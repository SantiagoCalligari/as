use yew::prelude::*;

//TODO: Card Container
fn card(cuerpo: String, Titulo: String, Imagen: String) -> Html {
    html! {
  <li class="card">
    <img src={Imagen}/>
    <h2>{Titulo}</h2>
    <p>{cuerpo}</p>
      </li>
    }
}

fn body() -> Html { 
    html! {
        <div class="cuerpo">
            <h1>{"Garchlinux"}</h1>
            <p>{"Discover Garchlinux, the lightweight and highly customizable Linux distribution based on Arch that offers a streamlined and personalized experience. With a carefully selected KDE desktop environment and pre-installed packages, Garchlinux is the perfect choice for both power users and beginners. The installation and updates are easy, and the comprehensive documentation is designed to help users with any issue that may arise. Garchlinux also offers a large software repository that provides users with the necessary tools and applications to meet their everyday computing needs. Join the Garch community today and enjoy the benefits of Arch without the hassle."}</p>
        </div>
    }
}
fn navbar() -> Html {
    html! {
        <div class="navbar">
            <a href="index.html" class="logo"/>
            <span>
            <a href="downloads.html" class="navbtn">{"Downloads"}</a>
            </span>
        </div> 
    }
}
fn requeriments() -> Html {
    html! {
      <li class="card">
        <img src="img/GarchaLinuxArgentina.png"/>
        <h2>{"System Requirements"}</h2>
        <ul>
            <li>{"A 64-bit processor"}</li>
            <li>{"4GB of RAM or more recommended"}</li>
            <li>{"10GB of free disk space or more recommended"}</li>
            <li>{"A graphics card capable of running KDE or other desktop environments"}</li>
            <li>{"A reliable and stable internet connection for downloading packages and updates"}</li>
        </ul>
        <p><small>{"Note: Garchlinux may work with lower specifications, but we recommend meeting the above requirements for optimal performance and user experience."}</small></p>
    </li>
    }

}


fn footer() -> Html {
    html! {
        <div class="footer-container">
            <div class="footer">
                <div class="redes">
                    <h3 class="contact">{"Contact Me"}</h3>
                    <a href="mailto:santiago@calligari.ar">{"santiago@garchlinux.ar"}</a>
                </div>
            </div>
        </div>
    }
}

#[function_component]
fn App() -> Html {
    html!{
        <>
            {navbar()}
            {body()}
            <ul class="card-wrapper">
            {card(String::from("Because Garchlinux offers a streamlined and personalized experience, with a carefully selected KDE desktop environment and pre-installed packages. Choose Garch for its robust and flexible platform, easy installation."), String::from("Why GarchLinux?"), String::from("img/GarchLinuxBootEntry.png"))}
            {requeriments()}
            </ul>
            {footer()}
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
