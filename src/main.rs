//! Awoo Tool by @wolfiestyle
use gtk::prelude::*;
use rand::Rng;

const AWOO: &str = "Awoo";
const BIG_AWOO: &str = "Awoooooooooo!";
const LONG_AWOO: &str = "awooooooooooooooooooo\noooooooooooooooooo...";
const START_HOWL: &str = "Click to start a howl";
const READY: &str = "Ready";
const AWOO_PROGRESS: [&str; 4] = ["Loading waffs...", "Disabling fines...", "*inhales*", "Awoooooooooo!"];

fn main() {
    let application = gtk::Application::builder().application_id("io.github.wolfiestyle.AwooTool").build();
    application.connect_activate(app_main);
    application.run();
}

fn app_main(app: &gtk::Application) {
    let spinner = gtk::Spinner::new();
    spinner.set_no_show_all(true);

    let msg = gtk::Label::new(START_HOWL.into());

    let hbox = gtk::Box::builder().spacing(5).halign(gtk::Align::Center).build();
    hbox.add(&spinner);
    hbox.add(&msg);

    let button = gtk::Button::with_label(AWOO);

    let progbar = gtk::ProgressBar::builder().text(READY).show_text(true).build();

    let vbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin(30)
        .spacing(10)
        .build();
    vbox.add(&button);
    vbox.add(&hbox);
    vbox.pack_end(&progbar, false, false, 0);

    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Awoo Tool 0.2-beta")
        .default_width(320)
        .default_height(200)
        .build();
    window.add(&vbox);

    let win = window.clone();
    button.connect_clicked(move |this| {
        this.set_sensitive(false);
        msg.set_text("Awooing...");
        spinner.show();
        spinner.start();
        progbar.set_text("Initializing...".into());

        let window_ = win.clone();
        let progbar_ = progbar.clone();
        let spinner_ = spinner.clone();
        let button_ = this.clone();
        let msg_ = msg.clone();
        let mut state = 0;
        glib::timeout_add_seconds_local(1, move || {
            if state < AWOO_PROGRESS.len() {
                progbar_.set_text(AWOO_PROGRESS[state].into());
                state += 1;
                progbar_.set_fraction(state as f64 / AWOO_PROGRESS.len() as f64);
                glib::Continue(true)
            } else if state == AWOO_PROGRESS.len() {
                state += 1;
                glib::Continue(true)
            } else {
                for _ in 0..10 {
                    make_awoo_dialog(&window_);
                }
                spinner_.stop();
                spinner_.hide();
                msg_.set_text(START_HOWL);
                progbar_.set_text(READY.into());
                progbar_.set_fraction(0.0);
                button_.set_sensitive(true);
                state = 0;
                glib::Continue(false)
            }
        });
    });

    window.show_all();
}

fn make_awoo_dialog(window: &gtk::ApplicationWindow) -> gtk::MessageDialog {
    let dlg = gtk::MessageDialog::builder()
        .title(AWOO)
        .buttons(gtk::ButtonsType::Ok)
        .text(BIG_AWOO)
        .secondary_text(LONG_AWOO)
        .build();

    dlg.set_transient_for(Some(window));
    dlg.connect_response(|w, _| w.close());

    let (x, y) = window.position();
    let mut rng = rand::thread_rng();
    dlg.move_(x + rng.gen_range(-50..=100), y + rng.gen_range(-50..=100));
    dlg.show();
    dlg
}
