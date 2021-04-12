use vtest_on_new_windows::bindings::{
    Windows::Data::Xml::Dom::*,
    Windows::Win32::SystemServices::{CreateEventW, SetEvent, WaitForSingleObject, PWSTR},
    Windows::Win32::WindowsAndMessaging::{MessageBoxA, HWND, MESSAGEBOX_STYLE},
    Windows::Win32::WindowsProgramming::CloseHandle,
};

fn main() -> windows::Result<()> {
    let doc = XmlDocument::new()?;
    doc.LoadXml("<html>hello world</html>")?;

    let root = doc.DocumentElement()?;
    assert!(root.NodeName()? == "html");
    assert!(root.InnerText()? == "hello world");

    unsafe {
        let event = CreateEventW(std::ptr::null_mut(), true, false, PWSTR::NULL);
        SetEvent(event).ok()?;
        WaitForSingleObject(event, 0);
        CloseHandle(event).ok()?;

        MessageBoxA(HWND(0), "Text", "Caption", MESSAGEBOX_STYLE::MB_OK);
    }

    Ok(())
}
