use yew::prelude::*;
use crate::patterns::Pattern;

#[derive(Properties, PartialEq)]
pub struct EditorProps {
    #[prop_or_default]
    pub on_pattern_change: Callback<Pattern>,
}

#[function_component(Editor)]
pub fn editor(props: &EditorProps) -> Html {
    let code = use_state(String::new);
    
    let oninput = {
        let code = code.clone();
        let on_pattern_change = props.on_pattern_change.clone();
        
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            code.set(value.clone());
            
            if let Ok(pattern) = Pattern::parse(&value) {
                on_pattern_change.emit(pattern);
            }
        })
    };
    
    html! {
        <div class="editor">
            <textarea
                class="code-input"
                placeholder="Enter your pattern (e.g., s('bd sd hh*2'))"
                oninput={oninput}
                value={(*code).clone()}
            />
        </div>
    }
} 