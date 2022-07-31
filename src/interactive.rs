use color_eyre::{eyre::WrapErr, Result};

pub fn select<T: ToString>(prompt: &str, items: &[T]) -> Result<Option<usize>> {
    let result = dialoguer::FuzzySelect::with_theme(&dialoguer::theme::ColorfulTheme {
        fuzzy_match_highlight_style: dialoguer::console::Style::new()
            .for_stderr()
            .yellow()
            .bold(),
        active_item_style: dialoguer::console::Style::new().for_stderr(),
        ..Default::default()
    })
    .items(items)
    .with_prompt(prompt)
    .default(0)
    .interact_opt()
    .wrap_err("Unable to make a selection")?;
    Ok(result)
}
