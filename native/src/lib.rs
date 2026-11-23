use jni::sys::{jint, jlong};
use renderdoc::{DevicePointer, RenderDoc, V110};

#[macro_export]
macro_rules! jenderdoc_fn {
    (
        $namespace:ident, ($env:ident, $class:ident) => {
            $(
                fn $name:ident($($pname:ident: $pty:ty),*) $(-> $ret:ty)? $code:block
            )*
        }
    ) => {
        paste::paste! {
            #[allow(dead_code, non_snake_case)]
            fn $namespace() {}

            $(
                #[allow(unused, unused_mut)]
                #[unsafe(no_mangle)]
                pub unsafe extern "system" fn [<Java_ $namespace _ $name>](
                    mut $env: jni::EnvUnowned,
                    $class: jni::objects::JClass,
                    $($pname: $pty,)*
                ) $(-> $ret)? {
                    #[allow(dead_code, non_snake_case)]
                    fn $name() {}
                    $code
                }
            )*
        }
    };
}

fn null_device() -> impl Into<DevicePointer> {
    #[cfg(unix)]
    {
        std::ptr::null_mut()
    }
    #[cfg(windows)]
    {
        std::ptr::null_mut() as *mut winapi::um::d3d11::ID3D11Device
    }
}

struct RenderDocApi {
    api: RenderDoc<V110>,
}

// pl epsi is that you?
jenderdoc_fn!(pl_epsi_jenderdoc_JenderDoc, (env, class) => {
    fn createRenderDoc() -> jlong {
        let rd = RenderDoc::<V110>::new();
        let rd = match rd {
            Ok(api) => RenderDocApi { api },
            Err(e) => {
                eprintln!("jenderdoc failed: '{e}' :wilted_rose:");
                return 0;
            }
        };

        Box::into_raw(Box::new(rd)).addr() as jlong
    }

    fn freeRenderDoc(rd: jlong) {
        unsafe {
            let _ = Box::from_raw(rd.cast_unsigned() as *mut RenderDocApi);
        }
    }

    fn startCapture(rd: jlong) {
        let rd = unsafe { &mut *(rd.cast_unsigned() as *mut RenderDocApi) };
        rd.api.start_frame_capture(
            null_device(),
            std::ptr::null_mut()
        )
    }

    fn endCapture(rd: jlong) {
        let rd = unsafe { &mut *(rd.cast_unsigned() as *mut RenderDocApi) };
        rd.api.end_frame_capture(
            null_device(),
            std::ptr::null_mut()
        )
    }

    fn captureNext(rd: jlong) {
        let rd = unsafe { &mut *(rd.cast_unsigned() as *mut RenderDocApi) };
        rd.api.trigger_capture();
    }

    fn captureFrames(rd: jlong, count: jint) {
        let rd = unsafe { &mut *(rd.cast_unsigned() as *mut RenderDocApi) };
        rd.api.trigger_multi_frame_capture(count as u32);
    }

    fn captureOption(rd: jlong, option: jint, value: jint) {
        let rd = unsafe { &mut *(rd.cast_unsigned() as *mut RenderDocApi) };
        rd.api.set_capture_option_u32(unsafe { std::mem::transmute(option.cast_unsigned()) }, value.cast_unsigned());
    }
});