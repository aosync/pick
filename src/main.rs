extern crate x11rb;

use std::error::Error;

use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;

extern crate colors_transform;
use colors_transform::Rgb;

extern crate colored;
use colored::*;

fn get_root<C: Connection>(conn: &C, snum: usize) -> Window {
    let default_screen = &conn.setup().roots[snum];
    default_screen.root
}

fn get_pointer_pos<C: Connection>(conn: &C, w: Window) -> Result<(i16, i16), Box<dyn Error>> {
    let cookie = query_pointer(conn, w)?;
    let reply = cookie.reply()?;
    Ok((reply.root_x, reply.root_y))
}

fn main() {
    let (conn, snum) = x11rb::connect(None).unwrap();
    let root = get_root(&conn, snum);
    grab_button(&conn, false, root, EventMask::ButtonPress as u16, GrabMode::Async, GrabMode::Async, root, 0 as u16, ButtonIndex::M1, 0 as u16).unwrap().check().unwrap();
    loop {
        conn.wait_for_event().unwrap();
        let (x, y) = get_pointer_pos(&conn, root).unwrap();
        let cookie = get_image(&conn, ImageFormat::ZPixmap, root, x, y, 1, 1, !0 as u32).unwrap();
        let reply = cookie.reply().unwrap();
        let rgb = Rgb::from(reply.data[2] as f32, reply.data[1] as f32, reply.data[0] as f32);
        println!("{} {} {}\n{}\n{}\n", reply.data[2], reply.data[1], reply.data[0], rgb.to_css_hex_string(), "       ".on_truecolor(reply.data[2], reply.data[1], reply.data[0]));
    }
}