use arboard::Clipboard;
use notify_rust::Notification;
use rust_bert::pipelines::summarization::SummarizationModel;

fn main() {
    let mut ctx = Clipboard::new().unwrap();
    let clip_txt = ctx.get_text().unwrap_or_else(|_| String::from(""));

    let model = SummarizationModel::new(Default::default()).unwrap();
    let input = vec![clip_txt];
    let output = model.summarize(&input).join("\n\n");
    println!("{}", &output);
    match ctx.set_text(&output) {
        Ok(_) => Notification::new()
            .summary("Summary Copied")
            .body(&output)
            .show()
            .unwrap(),
        Err(_) => Notification::new()
            .summary("Summary Copy Failed")
            .body(&output)
            .show()
            .unwrap(),
    };
}
