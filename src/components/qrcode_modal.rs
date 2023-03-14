use yew::{classes, function_component, html, Callback, Html, Properties, UseStateHandle};

#[derive(Properties, PartialEq)]
pub struct QrcodeModalProps {
    pub activeqrcode: UseStateHandle<bool>,
}

#[function_component(QrcodeModal)]
pub fn qrcode_modal(props: &QrcodeModalProps) -> Html {
    let close_on_pressed = {
        let activeqrcode = props.activeqrcode.clone();

        Callback::from(move |_| {
            activeqrcode.set(!*activeqrcode);
        })
    };

    let overlay = if *props.activeqrcode {
        html!( <div onclick={close_on_pressed.clone()} class="bg-black opacity-60 fixed top-16 right-0 bottom-0 left-0 z-30"></div>  )
    } else {
        html!()
    };

    let modal_style = "fixed top-32 bottom-16 inset-x-[16.66vw] z-40 w-2/3 p-8 bg-slate-700 \
                             rounded-lg flex flex-col gap-4 justify-center items-center transition-transform -translate-y-[100vh] duration-700";
    let modal_visible = "translate-y-0";

    html! {
        <>
          {overlay}

          <div class={classes!(modal_style, if *props.activeqrcode { modal_visible } else { "" })}>
            <button onclick={close_on_pressed} class="absolute top-4 right-4">
              <svg class="w-4 stroke-2 stroke-current hover:stroke-sky-500 ease-in duration-300" viewBox="0 0 16 16">
                <path d="M2.146 2.854a.5.5 0 1 1 .708-.708L8 7.293l5.146-5.147a.5.5 0 0 1 .708.708L8.707 \
                8l5.147 5.146a.5.5 0 0 1-.708.708L8 8.707l-5.146 5.147a.5.5 0 0 1-.708-.708L7.293 8 2.146 2.854Z" />
              </svg>
            </button>

            <span>{"Comparte este sitio:"}</span>
            <img src="img/qr-code.svg" class="bg-white rounded-lg w-4/5 md:max-h-[60vh]" alt="Qr Code" />
          </div>
        </>
    }
}

/*
 * https://stackoverflow.com/questions/42516502/svg-stroke-width-not-working
 * https://css-tricks.com/almanac/properties/s/stroke-width/
 * https://tailwindcss.com/docs/stroke-width
 */
