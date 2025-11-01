use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use yew::prelude::*;
use yew_hooks::use_interval;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub date: AttrValue,
    pub time: AttrValue,
}

#[function_component(Countdown)]
pub fn countdown(props: &Props) -> Html {
    let date = NaiveDateTime::new(
        NaiveDate::parse_from_str(&props.date, "%Y-%m-%d").unwrap_or_default(),
        NaiveTime::parse_from_str(&props.time, "%H:%M:%S").unwrap_or_default(),
    );
    let current_time = use_state(|| chrono::Local::now().naive_local());

    {
        let current_time = current_time.clone();
        use_interval(
            move || {
                current_time.set(chrono::Local::now().naive_local());
            },
            1000,
        );
    }

    let time_difference = date.signed_duration_since(*current_time);
    let days = time_difference.num_days().abs();
    let hours = (time_difference.num_hours() % 24).abs();
    let minutes = (time_difference.num_minutes() % 60).abs();
    let seconds = (time_difference.num_seconds() % 60).abs();

    html! {
        <div class="flex gap-5">
            <div>
                <span class="font-mono text-4xl" style="line-height: 1em;">
                    <span class="inline-block" aria-live="polite" aria-label={days.to_string()}>{ days }</span>
                </span>
                { "days" }
            </div>
            <div>
                <span class="countdown font-mono text-4xl">
                    <span style={ format!("--value:{};", hours) } aria-live="polite" aria-label={hours.to_string()}>{ hours }</span>
                </span>
                { "hours" }
            </div>
            <div>
                <span class="countdown font-mono text-4xl">
                    <span style={ format!("--value:{};", minutes) } aria-live="polite" aria-label={minutes.to_string()}>{ minutes }</span>
                </span>
                { "min" }
            </div>
            <div>
                <span class="countdown font-mono text-4xl">
                    <span style={ format!("--value:{};", seconds) } aria-live="polite" aria-label={seconds.to_string()}>{ seconds }</span>
                </span>
                { "sec" }
            </div>
        </div>
    }
}
