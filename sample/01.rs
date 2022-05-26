#[link(name="user32")]
extern "stdcall" {
    fn MessageBoxA(_: i32, _: i32, _: i32, _: u32) -> i32;
}
fn hello_world() -> () {
    unsafe { MessageBoxA(0, "Hello World!".as_ptr() as i32, "title".as_ptr() as i32, 0) };
}
