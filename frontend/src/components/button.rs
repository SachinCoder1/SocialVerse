use dioxus::prelude::*;

#[derive(Props)]
pub struct ButtonProps<'a> {
    #[props(default = "default")]
    pub variant: &'a str,
    #[props(default = "default")]
    pub size: &'a str,
    #[props(default = false)]
    pub loading: bool,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default = "button")]
    pub button_type: &'a str,
    pub children: Element<'a>,
}

pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element<'a> {
    let ButtonProps {
        variant,
        size,
        loading,
        disabled,
        button_type,
        children,
    } = cx.props;

    let variant_class = match *variant {
        "default" => "bg-primary text-primary-foreground hover:bg-primary/90",
        "destructive" => "bg-destructive text-destructive-foreground hover:bg-destructive/90",
        "outline" => {
            "border border-input bg-background hover:bg-accent hover:text-accent-foreground"
        }
        "secondary" => "bg-secondary text-secondary-foreground hover:bg-secondary/80",
        "ghost" => "hover:bg-accent hover:text-accent-foreground",
        "link" => "text-primary underline-offset-4 hover:underline",
        _ => "bg-primary text-primary-foreground hover:bg-primary/90",
    };

    let size_class = match *size {
        "sm" => "h-9 rounded-md px-3",
        "lg" => "h-11 rounded-md px-8",
        "icon" => "h-10 w-10",
        _ => "h-10 px-4 py-2",
    };

    let button_class = format!(
        "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 {} {}",
        variant_class, size_class
    );

    cx.render(rsx! {
        button {
            r#type: "{button_type}",
            class: "{button_class}",
            disabled: "{*disabled || *loading}",
            if *loading {
                rsx!(span { class: "loader", "Loading..." }) // Customize the loader as needed
            }
            children
        }
    })
}
