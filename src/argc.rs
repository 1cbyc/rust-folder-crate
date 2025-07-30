use clap::{crate_version, App, Arg};
pub fn argc_app() -> App<'static, 'static> {
    App::new("fold-dif")
        .version(&crate_version!()[..])
        .about("Sophisticated directory information CLI tool")
        .arg(
            Arg::with_name("nums")
                .help("Number of biggest files to display")
                .short("n")
                .takes_value(true),
        )
}
