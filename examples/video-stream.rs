extern crate ffmpeg_the_third as ffmpeg;
use ffmpeg::ffi::*;

/// 各参数说明：
//
// preset（预设）选项：
//
// ultrafast: 最快编码速度，但质量最低
// superfast: 非常快的编码速度
// veryfast: 很快的编码速度
// faster: 较快的编码速度
// fast: 快速编码
// medium: 默认值，平衡速度和质量
// slow: 慢速编码，更好的质量
// slower: 更慢的编码，更好的质量
// veryslow: 最慢的编码，最好的质量
//
// profile（配置）选项：
// baseline: 基本配置，适合移动设备
// main: 主要配置，适合标准清晰度
// high: 高级配置，适合高清视频（默认）
//
// tune（调优）选项：
// film: 适合电影内容
// animation: 适合动画内容
// grain: 保留胶片颗粒感
// stillimage: 适合静态图像
// fastdecode: 快速解码
// zerolatency: 零延迟，适合实时编码
//
// 建议：
// 如果是实时编码，使用 "ultrafast" 或 "superfast" preset
// 如果是离线编码，可以使用 "medium" 或更慢的 preset
// 对于网络流，"zerolatency" tune 选项很有用
// 如果不确定，使用默认的 "medium" preset 和 "high" profile 就可以

use std::ptr;
use std::path::Path;
use std::ffi::CString;

// 首先定义必要的类型和 trait
pub trait CodecType {
    fn get_type() -> AVMediaType;
}

pub struct Video;
impl CodecType for Video {
    fn get_type() -> AVMediaType {
        AVMediaType::AVMEDIA_TYPE_VIDEO
    }
}

// 编解码器结构体
pub struct Codec<T: CodecType> {
    ptr: *const AVCodec,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: CodecType> Codec<T> {
    pub fn as_ptr(&self) -> *const AVCodec {
        self.ptr
    }
}

// 流结构体
pub struct Stream {
    ptr: *mut AVStream,
    codec_ctx: *mut AVCodecContext,  // 添加编解码器上下文
}

impl Stream {
    pub fn as_mut_ptr(&self) -> *mut AVStream {
        self.ptr
    }

    pub fn codec_ctx(&mut self) -> *mut AVCodecContext {
        self.codec_ctx
    }
}

// 格式上下文结构体
pub struct FormatContext {
    ptr: *mut AVFormatContext,
}

impl FormatContext {

    /// 创建新的格式上下文用于输出
    pub fn new<P: AsRef<Path>>(filename: P) -> Result<Self, Error> {
        unsafe {
            let mut ctx: *mut AVFormatContext = ptr::null_mut();

            // 将文件名转换为 CString
            let c_filename = match CString::new(filename.as_ref().to_str().ok_or(Error::InvalidFilename)?) {
                Ok(s) => s,
                Err(_) => return Err(Error::InvalidFilename),
            };

            // 创建输出格式上下文
            let ret = avformat_alloc_output_context2(
                &mut ctx,
                ptr::null_mut(), // 让 FFmpeg 根据文件名猜测格式
                ptr::null(),     // 格式名称，null 表示自动检测
                c_filename.as_ptr()
            );

            if ret < 0 || ctx.is_null() {
                return Err(Error::FormatContextCreation);
            }

            // 打开输出文件
            let ret = avio_open(
                &mut (*ctx).pb,
                c_filename.as_ptr(),
                AVIO_FLAG_WRITE as i32
            );

            if ret < 0 {
                avformat_free_context(ctx);
                return Err(Error::FileOpen);
            }

            Ok(FormatContext { ptr: ctx })
        }
    }

    pub fn add_stream<T: CodecType>(&mut self, codec: &Codec<T>) -> Result<Stream, Error> {
        unsafe {
            // 创建新流
            let stream_ptr = avformat_new_stream(self.ptr, ptr::null());  // 不传递 codec
            if stream_ptr.is_null() {
                return Err(Error::StreamCreation);
            }

            // 创建编解码器上下文
            let mut codec_ctx = avcodec_alloc_context3(codec.as_ptr());
            if codec_ctx.is_null() {
                return Err(Error::CodecContextAllocation);
            }

            // 设置基本参数
            (*codec_ctx).width = 1920;
            (*codec_ctx).height = 1080;
            (*codec_ctx).time_base = AVRational { num: 1, den: 30 };
            (*codec_ctx).framerate = AVRational { num: 30, den: 1 };
            (*codec_ctx).pix_fmt = AVPixelFormat::AV_PIX_FMT_YUV420P;
            (*codec_ctx).bit_rate = 400_000;  // 400 kbps

            // 创建选项字典
            let mut opts: *mut AVDictionary = ptr::null_mut();

            // 设置 x264 参数
            av_dict_set(&mut opts,
                        CString::new("preset").unwrap().as_ptr(),
                        CString::new("medium").unwrap().as_ptr(),
                        0
            );

            // 修改这里，将 profile 值用引号括起来
            av_dict_set(&mut opts,
                        CString::new("profile:v").unwrap().as_ptr(),
                        CString::new("high").unwrap().as_ptr(),
                        0
            );

            av_dict_set(&mut opts,
                        CString::new("tune").unwrap().as_ptr(),
                        CString::new("zerolatency").unwrap().as_ptr(),
                        0
            );

            // 打开编解码器
            let result = avcodec_open2(codec_ctx, codec.as_ptr(), &mut opts);

            // 清理选项字典
            av_dict_free(&mut opts);

            if result < 0 {
                avcodec_free_context(&mut codec_ctx);
                return Err(Error::CodecOpen);
            }

            // 复制编解码器参数到流
            avcodec_parameters_from_context((*stream_ptr).codecpar, codec_ctx);

            Ok(Stream {
                ptr: stream_ptr,
                codec_ctx: codec_ctx,
            })
        }
    }

    pub fn with_format<P: AsRef<Path>>(filename: P, format_name: &str) -> Result<Self, Error> {
        unsafe {
            let mut ctx: *mut AVFormatContext = ptr::null_mut();

            let c_filename = match CString::new(filename.as_ref().to_str().ok_or(Error::InvalidFilename)?) {
                Ok(s) => s,
                Err(_) => return Err(Error::InvalidFilename),
            };

            let c_format = match CString::new(format_name) {
                Ok(s) => s,
                Err(_) => return Err(Error::InvalidFilename),
            };

            // 使用指定的格式创建上下文
            let ret = avformat_alloc_output_context2(
                &mut ctx,
                ptr::null_mut(),
                c_format.as_ptr(),
                c_filename.as_ptr()
            );

            if ret < 0 || ctx.is_null() {
                return Err(Error::FormatContextCreation);
            }

            let ret = avio_open(
                &mut (*ctx).pb,
                c_filename.as_ptr(),
                AVIO_FLAG_WRITE as i32
            );

            if ret < 0 {
                avformat_free_context(ctx);
                return Err(Error::FileOpen);
            }

            Ok(FormatContext { ptr: ctx })
        }
    }

    /// 获取可变指针
    pub fn as_mut_ptr(&mut self) -> *mut AVFormatContext {
        self.ptr
    }

    /// 写入文件头
    pub fn write_header(&mut self) -> Result<(), Error> {
        unsafe {
            let ret = avformat_write_header(self.ptr, ptr::null_mut());
            if ret < 0 {
                return Err(Error::WriteHeader);
            }
            Ok(())
        }
    }

    /// 写入文件尾
    pub fn write_trailer(&mut self) -> Result<(), Error> {
        unsafe {
            let ret = av_write_trailer(self.ptr);
            if ret < 0 {
                return Err(Error::WriteTrailer);
            }
            Ok(())
        }
    }
}

// 实现 Drop 以确保资源被正确释放
impl Drop for FormatContext {
    fn drop(&mut self) {
        unsafe {
            if !self.ptr.is_null() {
                if !(*self.ptr).pb.is_null() {
                    avio_closep(&mut (*self.ptr).pb);
                }
                avformat_free_context(self.ptr);
            }
        }
    }
}

// 为 Stream 实现 Drop 以释放资源
impl Drop for Stream {
    fn drop(&mut self) {
        unsafe {
            if !self.codec_ctx.is_null() {
                avcodec_free_context(&mut self.codec_ctx);
            }
        }
    }
}

// 编解码器查找函数
impl<T: CodecType> Codec<T> {
    pub fn find(codec_id: AVCodecID) -> Result<Self, Error> {
        unsafe {
            let ptr = avcodec_find_encoder(codec_id);
            if ptr.is_null() {
                return Err(Error::CodecOpen);
            }

            Ok(Codec {
                ptr,
                _phantom: std::marker::PhantomData,
            })
        }
    }
}

// 使用示例
fn encode_video(seconds: i32) -> Result<(), Error> {
    const WIDTH: i32 = 1920;    // 视频宽度
    const HEIGHT: i32 = 1080;   // 视频高度
    const FPS: i32 = 30;        // 视频帧率
    let total_frames: i32 = seconds * FPS;    // 总帧数

    // 创建格式上下文
    let mut format_ctx = FormatContext::new("/tmp/output.mp4")
        .expect("Failed to create format context");

    // 创建视频流
    let codec = Codec::<Video>::find(AVCodecID::AV_CODEC_ID_H264)?;
    let mut stream = format_ctx.add_stream(&codec)?;

    unsafe {
        // 设置流的时基
        (*stream.ptr).time_base = AVRational { num: 1, den: FPS };
        // 设置流的帧率
        (*stream.ptr).r_frame_rate = AVRational { num: FPS, den: 1 };
        (*stream.ptr).avg_frame_rate = AVRational { num: FPS, den: 1 };
    }

    format_ctx.write_header()?;

    unsafe {
        let mut frame = av_frame_alloc();
        let mut pkt = av_packet_alloc();

        (*frame).width = WIDTH;
        (*frame).height = HEIGHT;
        (*frame).format = AVPixelFormat::AV_PIX_FMT_YUV420P as i32;

        // 使用与流相同的时基
        (*frame).pts = 0;               // 初始显示时间戳
        (*frame).time_base = AVRational { num: 1, den: FPS };
        (*frame).duration = 1;          // 在当前时基下的持续时间

        let ret = av_frame_get_buffer(frame, 0);
        if ret < 0 {
            av_frame_free(&mut frame);
            av_packet_free(&mut pkt);
            return Err(Error::BufferAllocation);
        }

        for frame_idx in 0..total_frames {
            let ret = av_frame_make_writable(frame);
            if ret < 0 {
                break;
            }

            // 填充 Y 平面 (亮度)
            let y_ptr = (*frame).data[0];
            let y_linesize = (*frame).linesize[0];
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    // 创建一个随时间变化的渐变效果
                    let color_value = (((frame_idx as f32 / total_frames as f32) * 255.0) as i32)
                        .max(0)
                        .min(255) as u8;
                    *y_ptr.offset((y * y_linesize + x) as isize) = color_value;
                }
            }

            // 填充 U/V 平面 (色度)
            let u_ptr = (*frame).data[1];
            let v_ptr = (*frame).data[2];
            let uv_linesize = (*frame).linesize[1];
            for y in 0..(HEIGHT/2) {
                for x in 0..(WIDTH/2) {
                    *u_ptr.offset((y * uv_linesize + x) as isize) = 128;
                    *v_ptr.offset((y * uv_linesize + x) as isize) = 128;
                }
            }

            // 设置正确的时间戳
            (*frame).pts = frame_idx as i64;
            (*frame).pkt_dts = frame_idx as i64;
            // 设置持续时间为1个时基单位
            (*frame).duration = 1;

            let ret = avcodec_send_frame(stream.codec_ctx(), frame);
            if ret < 0 {
                break;
            }

            // 接收编码后的包
            while avcodec_receive_packet(stream.codec_ctx(), pkt) >= 0 {
                // 设置包的时间戳
                (*pkt).pts = (*frame).pts;
                (*pkt).dts = (*frame).pkt_dts;
                (*pkt).duration = (*frame).duration;

                // 转换时间戳到流的时基
                av_packet_rescale_ts(
                    pkt,
                    (*frame).time_base,
                    (*stream.ptr).time_base
                );

                // 写入包
                av_interleaved_write_frame(format_ctx.as_mut_ptr(), pkt);
                av_packet_unref(pkt);
            }
        }

        // 冲刷编码器
        avcodec_send_frame(stream.codec_ctx(), ptr::null());
        while avcodec_receive_packet(stream.codec_ctx(), pkt) >= 0 {
            // 确保最后的包也有正确的时间戳
            av_packet_rescale_ts(
                pkt,
                (*frame).time_base,
                (*stream.ptr).time_base
            );

            av_interleaved_write_frame(format_ctx.as_mut_ptr(), pkt);
            av_packet_unref(pkt);
        }

        av_frame_free(&mut frame);
        av_packet_free(&mut pkt);
    }

    format_ctx.write_trailer()?;
    Ok(())
}

// 扩展错误类型
#[derive(Debug)]
pub enum Error {
    InvalidFilename,
    FormatContextCreation,
    FileOpen,
    WriteHeader,
    WriteTrailer,
    StreamCreation,
    CodecContextAllocation,
    CodecOpen,
    BufferAllocation,
    // ... 其他错误类型
}

// 使用示例
fn main() -> Result<(), Error> {

    let _ = encode_video(10);

    Ok(())
}
