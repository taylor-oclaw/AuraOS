//! Core-only framebuffer drawing primitives.

use core::cmp::{max, min};

#[inline]
fn draw_hline(
    buf: &mut [u32],
    stride: usize,
    w: i32,
    h: i32,
    x0: i32,
    x1: i32,
    y: i32,
    color: u32,
) {
    if buf.is_empty() || stride == 0 || w <= 0 || h <= 0 || y < 0 || y >= h {
        return;
    }

    let mut start = min(x0, x1);
    let mut end = max(x0, x1);

    if end < 0 || start >= w {
        return;
    }

    start = max(start, 0);
    end = min(end, w - 1);

    let Ok(y) = usize::try_from(y) else {
        return;
    };
    let Ok(start) = usize::try_from(start) else {
        return;
    };
    let Ok(end) = usize::try_from(end) else {
        return;
    };

    if start >= stride {
        return;
    }

    let end = min(end, stride.saturating_sub(1));
    if start > end {
        return;
    }

    let Some(row_start) = y.checked_mul(stride) else {
        return;
    };
    let Some(start_idx) = row_start.checked_add(start) else {
        return;
    };
    let Some(end_idx) = row_start.checked_add(end) else {
        return;
    };

    if start_idx >= buf.len() {
        return;
    }

    let end_idx = min(end_idx, buf.len() - 1);
    for pixel in &mut buf[start_idx..=end_idx] {
        *pixel = color;
    }
}

#[inline]
fn plot_circle_points(
    buf: &mut [u32],
    stride: usize,
    w: i32,
    h: i32,
    cx: i32,
    cy: i32,
    x: i32,
    y: i32,
    color: u32,
) {
    set_pixel(
        buf,
        stride,
        cx.saturating_add(x),
        cy.saturating_add(y),
        w,
        h,
        color,
    );
    set_pixel(
        buf,
        stride,
        cx.saturating_sub(x),
        cy.saturating_add(y),
        w,
        h,
        color,
    );
    set_pixel(
        buf,
        stride,
        cx.saturating_add(x),
        cy.saturating_sub(y),
        w,
        h,
        color,
    );
    set_pixel(
        buf,
        stride,
        cx.saturating_sub(x),
        cy.saturating_sub(y),
        w,
        h,
        color,
    );
    set_pixel(
        buf,
        stride,
        cx.saturating_add(y),
        cy.saturating_add(x),
        w,
        h,
        color,
    );
    set_pixel(
        buf,
        stride,
        cx.saturating_sub(y),
        cy.saturating_add(x),
        w,
        h,
        color,
    );
    set_pixel(
        buf,
        stride,
        cx.saturating_add(y),
        cy.saturating_sub(x),
        w,
        h,
        color,
    );
    set_pixel(
        buf,
        stride,
        cx.saturating_sub(y),
        cy.saturating_sub(x),
        w,
        h,
        color,
    );
}

pub fn set_pixel(buf: &mut [u32], stride: usize, x: i32, y: i32, w: i32, h: i32, color: u32) {
    if buf.is_empty() || stride == 0 || w <= 0 || h <= 0 || x < 0 || y < 0 || x >= w || y >= h {
        return;
    }

    let Ok(x) = usize::try_from(x) else {
        return;
    };
    let Ok(y) = usize::try_from(y) else {
        return;
    };

    if x >= stride {
        return;
    }

    let Some(row_start) = y.checked_mul(stride) else {
        return;
    };
    let Some(idx) = row_start.checked_add(x) else {
        return;
    };

    if idx < buf.len() {
        buf[idx] = color;
    }
}

pub fn draw_line(
    buf: &mut [u32],
    stride: usize,
    w: i32,
    h: i32,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    color: u32,
) {
    if buf.is_empty() || stride == 0 || w <= 0 || h <= 0 {
        return;
    }

    let mut x0 = i64::from(x0);
    let mut y0 = i64::from(y0);
    let x1 = i64::from(x1);
    let y1 = i64::from(y1);

    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        set_pixel(buf, stride, x0 as i32, y0 as i32, w, h, color);

        if x0 == x1 && y0 == y1 {
            break;
        }

        let e2 = err * 2;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

pub fn fill_rect(
    buf: &mut [u32],
    stride: usize,
    w: i32,
    h: i32,
    x: i32,
    y: i32,
    rw: i32,
    rh: i32,
    color: u32,
) {
    if buf.is_empty() || stride == 0 || w <= 0 || h <= 0 || rw <= 0 || rh <= 0 {
        return;
    }

    let x_start = max(x, 0);
    let y_start = max(y, 0);
    let x_end = min(x.saturating_add(rw), w);
    let y_end = min(y.saturating_add(rh), h);

    if x_start >= x_end || y_start >= y_end {
        return;
    }

    for yy in y_start..y_end {
        draw_hline(buf, stride, w, h, x_start, x_end - 1, yy, color);
    }
}

pub fn draw_rect(
    buf: &mut [u32],
    stride: usize,
    w: i32,
    h: i32,
    x: i32,
    y: i32,
    rw: i32,
    rh: i32,
    color: u32,
) {
    if buf.is_empty() || stride == 0 || w <= 0 || h <= 0 || rw <= 0 || rh <= 0 {
        return;
    }

    fill_rect(buf, stride, w, h, x, y, rw, 1, color);

    if rh > 1 {
        fill_rect(buf, stride, w, h, x, y.saturating_add(rh - 1), rw, 1, color);
    }

    if rh > 2 {
        fill_rect(buf, stride, w, h, x, y.saturating_add(1), 1, rh - 2, color);

        if rw > 1 {
            fill_rect(
                buf,
                stride,
                w,
                h,
                x.saturating_add(rw - 1),
                y.saturating_add(1),
                1,
                rh - 2,
                color,
            );
        }
    }
}

pub fn draw_circle(
    buf: &mut [u32],
    stride: usize,
    w: i32,
    h: i32,
    cx: i32,
    cy: i32,
    r: i32,
    color: u32,
) {
    if buf.is_empty() || stride == 0 || w <= 0 || h <= 0 || r < 0 {
        return;
    }

    if r == 0 {
        set_pixel(buf, stride, cx, cy, w, h, color);
        return;
    }

    let mut x = r;
    let mut y = 0;
    let mut decision = 1 - r;

    while x >= y {
        plot_circle_points(buf, stride, w, h, cx, cy, x, y, color);

        y += 1;
        if decision <= 0 {
            decision += 2 * y + 1;
        } else {
            x -= 1;
            decision += 2 * (y - x) + 1;
        }
    }
}

pub fn fill_circle(
    buf: &mut [u32],
    stride: usize,
    w: i32,
    h: i32,
    cx: i32,
    cy: i32,
    r: i32,
    color: u32,
) {
    if buf.is_empty() || stride == 0 || w <= 0 || h <= 0 || r < 0 {
        return;
    }

    if r == 0 {
        set_pixel(buf, stride, cx, cy, w, h, color);
        return;
    }

    let mut x = r;
    let mut y = 0;
    let mut decision = 1 - r;

    while x >= y {
        draw_hline(
            buf,
            stride,
            w,
            h,
            cx.saturating_sub(x),
            cx.saturating_add(x),
            cy.saturating_add(y),
            color,
        );
        draw_hline(
            buf,
            stride,
            w,
            h,
            cx.saturating_sub(x),
            cx.saturating_add(x),
            cy.saturating_sub(y),
            color,
        );
        draw_hline(
            buf,
            stride,
            w,
            h,
            cx.saturating_sub(y),
            cx.saturating_add(y),
            cy.saturating_add(x),
            color,
        );
        draw_hline(
            buf,
            stride,
            w,
            h,
            cx.saturating_sub(y),
            cx.saturating_add(y),
            cy.saturating_sub(x),
            color,
        );

        y += 1;
        if decision <= 0 {
            decision += 2 * y + 1;
        } else {
            x -= 1;
            decision += 2 * (y - x) + 1;
        }
    }
}
