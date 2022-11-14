#![feature(core_panic)]
#![feature(stdsimd)]
use core::panicking::const_panic_fmt;
use std::arch::x86_64::_mm_conflict_epi32;
use std::convert::Infallible;

use nvim_oxi::api::{self, opts::*, types::*, Window, Buffer};
use nvim_oxi::{self as oxi, Dictionary, Function, Object};
use nvim_oxi::print;
#[oxi::module]
fn calc() -> oxi::Result<Dictionary> {
    let add =
        Function::from_fn(|(a, b): (i32, i32)| Ok::<_, Infallible>(a + b));

    let multiply =
        Function::from_fn(|(a, b): (i32, i32)| Ok::<_, Infallible>(a * b));

    let compute = Function::from_fn(
        |(fun, a, b): (Function<(i32, i32), i32>, i32, i32)| fun.call((a, b)),
    );



    use std::cell::RefCell;
    use std::rc::Rc;

    let win: Rc<RefCell<Option<Window>>> = Rc::default();

    let v :Vec<String> = Vec::new();
    let yoinker = std::sync::Arc::new(std::sync::Mutex::new(v)); 
    let cb_yoink = yoinker.clone();
    let win_yoink = yoinker.clone();
    let mut buf = api::create_buf(false, true)?;

    //command and callback are mutually exclusive - no have both
    let opts = CreateAutocmdOpts::builder()
        //.command("echo 'hi there'")
        .callback( move |args: AutocmdCallbackArgs| {

            let ev_type: String= args.event;
            let res: Dictionary = api::get_vvar("event").unwrap();

            //let buf_name: String = res.get(&"regtype").unwrap().into_string_unchecked().to_string();
            let buf_obj = res.get(&"regtype").unwrap();
            let buf_name: String = buf_obj.clone().to_string();
            //oxi::print!("grabbing from buffer: {}", buf_name);
            let buf_data = api::call_function::<_, String>("getreg", (buf_name,));
            let val = buf_data.unwrap();
            let mut i = cb_yoink.lock().unwrap();
            //i.push("hi there".to_string());
            i.push(val);
            //
            Ok::<_, oxi::Error>(false)
        })
        //.patterns(["*"])
        .buffer(0)
        .build();



    let id = api::create_autocmd(["TextYankPost"], &opts);
    assert!(id.is_ok(), "{id:?}");


    let w = Rc::clone(&win);
    let open_window = Function::from_fn::<_, oxi::Error>(move |()| {
        if w.borrow().is_some() {
            api::err_writeln("Window is already open");
            return Ok(());
        }
        
        let mut buf = api::create_buf(false, true)?;
        //buf.set_lines(.., true, ["test"]);
        let i = win_yoink.lock().unwrap();
        let mut v :Vec<String> = Vec::new();
        for yank in i.iter() {
            let mut split = yank.split("\n");
            for s in split {
                v.push(s.to_string());
            }

        }

        buf.set_lines(.., true, v);
        oxi::print!("lines len {}", i.len());
        let config = WindowConfig::builder()
            .relative(WindowRelativeTo::Cursor)
            .height(20)
            .width(60)
            .row(1)
            .col(0)
            .build();

        let mut win = w.borrow_mut();
        *win = Some(api::open_win(&buf, false, &config)?);

        Ok(())
    });

    //Ok(Dictionary::from_iter([
    //]))

    Ok(Dictionary::from_iter([
        ("open_window", open_window),
    ]))
}
