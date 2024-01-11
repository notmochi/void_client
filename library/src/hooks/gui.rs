use std::sync::Once;

use crate::guilib::app::OpenGLApp;
use crate::util::logger::Logger;
use egui::Context;
use retour::static_detour;
use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress};
use windows::Win32::Foundation::{HWND, WPARAM, LPARAM, LRESULT};
use windows::core::{s, HRESULT};
use windows::Win32::Graphics::Gdi::{WindowFromDC, HDC};
use windows::Win32::UI::WindowsAndMessaging::{
    SetWindowLongPtrA, GWLP_WNDPROC, WNDPROC, WM_KEYUP, CallWindowProcA,
};

use win_key_codes::*;

pub type FnWglSwapBuffers = unsafe extern "stdcall" fn(HDC) -> HRESULT; // these lines of code are quite stolen, now sue me.

static_detour! {
    pub static WglSwapBuffersHook: unsafe extern "stdcall" fn(HDC) -> HRESULT;
}

static mut APP: OpenGLApp<i32> = OpenGLApp::new();
static mut OLD_WND_PROC: Option<WNDPROC> = None;
pub static mut GUI_OPEN: bool = true;

fn render_gui(ctx: &Context, _: &mut i32) {
    unsafe {
        if !GUI_OPEN { return };
        egui::containers::Window::new("void")
            .pivot(egui::Align2::CENTER_CENTER)
            .default_pos(egui::pos2(0.0, 0.0))
            .show(ctx, |ui| {
                let mut b: bool = true;
                ui.checkbox(&mut b, "GAy");
            });
    }
}

fn hooked(hdc: HDC) -> HRESULT {
    static mut INIT: Once = Once::new();
    unsafe {
        INIT.call_once(|| {
            Logger::log("Swap buffers hooked");
            let window = WindowFromDC(hdc);
            APP.init_default(hdc, window, render_gui);
            Logger::log("Egui initialized");
            OLD_WND_PROC = Some(std::mem::transmute(SetWindowLongPtrA(
                window,
                GWLP_WNDPROC,
                wnd_proc_hook as usize as _,
            )));
            Logger::log("Swapped winproc");
        });

        APP.render(hdc);
        WglSwapBuffersHook.call(hdc)
    }
}

pub unsafe fn init() {
    Logger::log("Retrieving opengl32.dll...");
    let module = GetModuleHandleA(s!("opengl32.dll").as_ptr() as *const i8);
    Logger::log_fmt(format_args!("{}{:p}", "Retrieved opengl32.dll: ", module));

    Logger::log("Getting wglSwapBuffers...");
    let wgl_swap_buffers_address: *const usize = GetProcAddress(module, s!("wglSwapBuffers").as_ptr() as *const i8) as _;
    let fn_wgl_swap_buffers: FnWglSwapBuffers = std::mem::transmute(wgl_swap_buffers_address);
    Logger::log_fmt(format_args!("{}{:p}", "Got wglSwapBuffers: ", wgl_swap_buffers_address));

    WglSwapBuffersHook
        .initialize(fn_wgl_swap_buffers, hooked)
        .unwrap()
        .enable()
        .unwrap();
}

pub unsafe fn stop() {
    WglSwapBuffersHook.disable().unwrap();
    let wnd_proc = OLD_WND_PROC.unwrap().unwrap();
    SetWindowLongPtrA(APP.get_window(), GWLP_WNDPROC, wnd_proc as usize as _);
}

unsafe extern "stdcall" fn wnd_proc_hook(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if APP.wnd_proc(msg, wparam, lparam) {
        return LRESULT(1);
    }
    if msg == WM_KEYUP {
        if wparam.0 as i16 == VK_RCONTROL as i16 {
            GUI_OPEN = !GUI_OPEN;
        }
    }
    CallWindowProcA(OLD_WND_PROC.unwrap(), hwnd, msg, wparam, lparam)
}