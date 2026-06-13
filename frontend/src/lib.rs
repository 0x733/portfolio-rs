use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::cell::RefCell;
use std::rc::Rc;

// Simple LCG random generator
static mut SEED: u32 = 987654321;

fn random() -> f64 {
    unsafe {
        SEED = SEED.wrapping_mul(1103515245).wrapping_add(12345);
        (SEED & 0x7fffffff) as f64 / 2147483647.0
    }
}

// Helper to set timeout
fn set_timeout<F>(closure: F, timeout_ms: i32)
where
    F: FnOnce() + 'static,
{
    let window = web_sys::window().unwrap();
    let cb = Closure::once_into_js(closure);
    window.set_timeout_with_callback_and_timeout_and_arguments_0(
        cb.as_ref().unchecked_ref(),
        timeout_ms,
    ).unwrap();
}

// Particle structure
struct Particle {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    radius: f64,
    opacity: f64,
    opacity_speed: f64,
}

impl Particle {
    fn new(width: f64, height: f64) -> Self {
        Particle {
            x: random() * width,
            y: random() * height,
            vx: 0.1 + random() * 0.4, // Move slowly right
            vy: (random() - 0.5) * 0.1, // Slight vertical drift
            radius: 0.5 + random() * 1.5,
            opacity: 0.1 + random() * 0.4,
            opacity_speed: (0.2 + random() * 0.8) * 0.005,
        }
    }

    fn update(&mut self, width: f64, height: f64) {
        self.x += self.vx;
        self.y += self.vy;

        // Pulse opacity
        self.opacity += self.opacity_speed;
        if self.opacity <= 0.05 || self.opacity >= 0.5 {
            self.opacity_speed = -self.opacity_speed;
        }

        // Reset if goes off-screen
        if self.x > width {
            self.x = 0.0;
            self.y = random() * height;
        }
        if self.y < 0.0 || self.y > height {
            self.y = random() * height;
        }
    }
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("no global window exists")?;
    let document = window.document().ok_or("should have a document on window")?;

    // 1. Preloader fade out
    if let Some(preloader) = document.get_element_by_id("preloader") {
        let preloader_el = preloader.dyn_into::<web_sys::HtmlElement>()?;
        set_timeout(move || {
            let style = preloader_el.style();
            style.set_property("transition", "opacity 0.4s ease").unwrap();
            style.set_property("opacity", "0").unwrap();
            
            let preloader_el_clone = preloader_el.clone();
            set_timeout(move || {
                preloader_el_clone.style().set_property("display", "none").unwrap();
            }, 400);
        }, 500);
    }

    // 2. Sticky Navbar scroll listener
    if let Some(nav) = document.query_selector(".navbar")? {
        let nav_el = nav.dyn_into::<web_sys::HtmlElement>()?;
        let window_clone = window.clone();
        
        let on_scroll = Closure::<dyn FnMut()>::new(move || {
            let scroll_y = window_clone.scroll_y().unwrap_or(0.0);
            let class_list = nav_el.class_list();
            if scroll_y >= 20.0 {
                class_list.add_1("sticky").unwrap();
            } else {
                class_list.remove_1("sticky").unwrap();
            }
        });
        
        window.add_event_listener_with_callback(
            "scroll",
            on_scroll.as_ref().unchecked_ref(),
        )?;
        on_scroll.forget(); // Keep memory alive
    }

    // 3. Mobile Navbar collapse toggle (Pure WASM/Rust toggler click)
    if let Some(toggler) = document.query_selector(".navbar-toggler")? {
        if let Some(nav_nav) = document.get_element_by_id("responsive-navbar-nav") {
            let nav_nav_el = nav_nav.dyn_into::<web_sys::HtmlElement>()?;
            let toggler_el = toggler.dyn_into::<web_sys::HtmlElement>()?;
            let toggler_el_clone = toggler_el.clone();
            
            let on_click = Closure::<dyn FnMut()>::new(move || {
                let class_list = nav_nav_el.class_list();
                let is_expanded = class_list.contains("show");
                if is_expanded {
                    class_list.remove_1("show").unwrap();
                    toggler_el_clone.set_attribute("aria-expanded", "false").unwrap();
                    toggler_el_clone.class_list().add_1("collapsed").unwrap();
                } else {
                    class_list.add_1("show").unwrap();
                    toggler_el_clone.set_attribute("aria-expanded", "true").unwrap();
                    toggler_el_clone.class_list().remove_1("collapsed").unwrap();
                }
            });
            
            toggler_el.add_event_listener_with_callback(
                "click",
                on_click.as_ref().unchecked_ref(),
            )?;
            on_click.forget();
        }
    }

    // 4. Typewriter Effect
    if let Some(typewriter) = document.get_element_by_id("typewriter") {
        let typewriter_el = typewriter.dyn_into::<web_sys::HtmlElement>()?;
        
        let strings = vec![
            "Systems & Security Engineer".to_string(),
            "Android App Developer".to_string(),
            "Open Source Contributor".to_string(),
            "Linux Automation Specialist".to_string(),
        ];
        
        // State using Rc and RefCell
        let state = Rc::new(RefCell::new((
            strings,
            0,    // current string index
            0,    // current character index
            false, // is deleting
            typewriter_el,
        )));
        
        let state_clone = state.clone();
        
        // Recursively defined function via RefCell
        let tick = Rc::new(RefCell::new(None));
        let tick_clone = tick.clone();
        
        *tick_clone.borrow_mut() = Some(Closure::<dyn FnMut()>::new(move || {
            let mut s = state_clone.borrow_mut();
            let strings = &s.0;
            let current_str_idx = s.1;
            let mut current_char_idx = s.2;
            let mut is_deleting = s.3;
            let el = &s.4;
            
            let current_text = &strings[current_str_idx];
            let mut next_timeout = 60; // default typing speed
            
            if is_deleting {
                current_char_idx -= 1;
                // Safely handle slicing of unicode strings
                let slice = if current_char_idx > 0 {
                    current_text.chars().take(current_char_idx).collect::<String>()
                } else {
                    "".to_string()
                };
                el.set_inner_html(&format!("{}<span class=\"Typewriter__cursor\">|</span>", slice));
                next_timeout = 30; // deleting is faster
                
                if current_char_idx == 0 {
                    is_deleting = false;
                    s.1 = (current_str_idx + 1) % strings.len(); // move to next string
                    next_timeout = 500; // wait before typing next
                }
            } else {
                current_char_idx += 1;
                let slice = current_text.chars().take(current_char_idx).collect::<String>();
                el.set_inner_html(&format!("{}<span class=\"Typewriter__cursor\">|</span>", slice));
                
                if current_char_idx == current_text.chars().count() {
                    is_deleting = true;
                    next_timeout = 1500; // pause at full text
                }
            }
            
            s.2 = current_char_idx;
            s.3 = is_deleting;
            
            // Re-schedule
            let window = web_sys::window().unwrap();
            let next_tick = tick.borrow();
            let next_tick_cb = next_tick.as_ref().unwrap();
            let next_tick_js: &JsValue = AsRef::<JsValue>::as_ref(next_tick_cb);
            let next_tick_func: &js_sys::Function = next_tick_js.unchecked_ref();
            window.set_timeout_with_callback_and_timeout_and_arguments_0(
                next_tick_func,
                next_timeout,
            ).unwrap();
        }));
        
        // Start the first tick
        let window = web_sys::window().unwrap();
        let first_tick = tick_clone.borrow();
        let first_tick_cb = first_tick.as_ref().unwrap();
        let first_tick_js: &JsValue = AsRef::<JsValue>::as_ref(first_tick_cb);
        let first_tick_func: &js_sys::Function = first_tick_js.unchecked_ref();
        window.set_timeout_with_callback_and_timeout_and_arguments_0(
            first_tick_func,
            100,
        ).unwrap();
    }

    // 5. Canvas Particle System
    if let Some(canvas_element) = document.get_element_by_id("particle-canvas") {
        let canvas = canvas_element.dyn_into::<web_sys::HtmlCanvasElement>()?;
        let ctx = canvas
            .get_context("2d")?
            .ok_or("2d context not supported")?
            .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
            
        // Set initial size
        let resize_canvas = {
            let canvas = canvas.clone();
            let window = window.clone();
            move || {
                let w = window.inner_width().unwrap().as_f64().unwrap();
                let h = window.inner_height().unwrap().as_f64().unwrap();
                canvas.set_width(w as u32);
                canvas.set_height(h as u32);
            }
        };
        
        resize_canvas();
        
        // Listen to resize
        let resize_canvas_clone = resize_canvas.clone();
        let on_resize = Closure::<dyn FnMut()>::new(move || {
            resize_canvas_clone();
        });
        window.add_event_listener_with_callback(
            "resize",
            on_resize.as_ref().unchecked_ref(),
        )?;
        on_resize.forget();
        
        // Create particles
        let num_particles = 100;
        let mut particles = Vec::with_capacity(num_particles);
        let w = canvas.width() as f64;
        let h = canvas.height() as f64;
        for _ in 0..num_particles {
            particles.push(Particle::new(w, h));
        }
        
        // Loop state
        let loop_state = Rc::new(RefCell::new((canvas, ctx, particles)));
        let loop_state_clone = loop_state.clone();
        
        let animate = Rc::new(RefCell::new(None));
        let animate_clone = animate.clone();
        
        *animate_clone.borrow_mut() = Some(Closure::<dyn FnMut()>::new(move || {
            let mut state = loop_state_clone.borrow_mut();
            let (ref canvas, ref ctx, ref mut particles) = *state;
            
            let w = canvas.width() as f64;
            let h = canvas.height() as f64;
            
            // Clear canvas
            ctx.clear_rect(0.0, 0.0, w, h);
            
            // Render and update particles
            for p in particles.iter_mut() {
                p.update(w, h);
                ctx.begin_path();
                ctx.arc(p.x, p.y, p.radius, 0.0, std::f64::consts::TAU).unwrap();
                ctx.set_fill_style(&JsValue::from_str(&format!("rgba(200, 137, 230, {})", p.opacity)));
                ctx.fill();
            }
            
            // Re-schedule next frame
            let window = web_sys::window().unwrap();
            let next_frame = animate.borrow();
            let next_frame_cb = next_frame.as_ref().unwrap();
            let next_frame_js: &JsValue = AsRef::<JsValue>::as_ref(next_frame_cb);
            let next_frame_func: &js_sys::Function = next_frame_js.unchecked_ref();
            window.request_animation_frame(next_frame_func).unwrap();
        }));
        
        // Trigger first frame
        let window = web_sys::window().unwrap();
        let first_frame = animate_clone.borrow();
        let first_frame_cb = first_frame.as_ref().unwrap();
        let first_frame_js: &JsValue = AsRef::<JsValue>::as_ref(first_frame_cb);
        let first_frame_func: &js_sys::Function = first_frame_js.unchecked_ref();
        window.request_animation_frame(first_frame_func).unwrap();
    }

    Ok(())
}
