use rocket::*;
use rocket::response::content::RawHtml;
use rocket::fs::{relative, FileServer};

#[get("/")]
fn index() -> RawHtml<String> {
    RawHtml(r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>About Me</title>
            <link href="https://fonts.googleapis.com/css2?family=Jost:wght@200;300;400&display=swap" rel="stylesheet">
            <link rel="icon" type="image/x-icon" href="/static/favicon.ico">
            <style>
                /* Color Palette */
                :root {
                    --background-color: #232932;
                    --primary-color: #232932;
                    --header-color: #283447;
                    --text-color: #e0e0e0;
                    --accent-color: #ED5145;
                    --gradient-start: #F27121;
                    --gradient-middle: #E94057;
                    --gradient-end: #8A2387;
                    --container-shadow: rgba(0, 0, 0, 0.3);
                }

                body {
                    font-family: 'Jost', sans-serif;
                    background-color: var(--background-color);
                    color: var(--text-color);
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    min-height: 100vh;
                    margin: 0;
                    padding: 20px;
                }
                .container {
                    text-align: center;
                    max-width: 800px;
                    margin: 0 auto;
                    box-shadow: 0px 8px 15px var(--container-shadow);
                    border-radius: 10px;
                    padding: 1rem;
                    background-color: var(--primary-color);
                }
                .header {
                    padding: 1rem;
                    background-color: var(--header-color);
                    border-radius: 10px;
                }
                h1 {
                    font-size: 2rem;
                }
                p {
                    font-size: 1rem;
                    line-height: 1.6;
                    color: var(--text-color);
                }
                .icon-wrapper {
                    position: relative;
                    display: inline-block;
                    margin: 1.5rem;
                }

                .icon {
                    width: 8rem;
                    height: 8rem;
                    transition: transform 0.3s;
                }

                .icon:hover {
                    transform: scale(1.1);
                }

                .skill-icons {
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    flex-wrap: wrap;
                    gap: 2rem;
                }

                .icon-wrapper::after {
                    content: attr(data-skill);
                    position: absolute;
                    top: 110%;
                    left: 50%;
                    transform: translateX(-50%);
                    min-width: 10rem;
                    padding: 0.5rem 1rem;
                    background-color: var(--header-color);
                    color: #fff;
                    font-size: 0.8rem;
                    border-radius: 0.5rem;
                    opacity: 0;
                    visibility: hidden;
                    transition: opacity 0.6s, visibility 0.6s;
                    z-index: 10;
                    text-align: center;
                }

                .icon-wrapper:hover::after {
                    opacity: 1;
                    visibility: visible;
                }
                .text-highlight {
                    color: var(--accent-color);
                }
                .timeline {
                    width: 90%;
                    margin: 2rem auto;
                    display: flex;
                    flex-direction: column;
                }
                .timeline-content {
                    padding: 20px;
                    background: var(--background-color);
                    box-shadow: 5px 5px 10px #1a1a1a, -5px -5px 10px #242424;
                    border-radius: 6px;
                    color: var(--text-color);
                    transition: 0.4s ease;
                    overflow-wrap: break-word;
                    margin: 1rem;
                }
                .timeline-component {
                    margin-bottom: 20px;
                    position: relative;
                }
                .timeline-circle {
                    position: absolute;
                    left: 50%;
                    transform: translateX(-50%);
                    width: 15px;
                    height: 15px;
                    border-radius: 50%;
                    background-image: linear-gradient(45deg, var(--gradient-start), var(--gradient-middle), var(--gradient-end));
                }
                h1 {
                    font-size: 2.5rem;
                }
                .timeline {
                    flex-direction: column;
                    position: relative;
                }
                .timeline::before {
                    content: '';
                    position: absolute;
                    left: 50%;
                    top: 0;
                    bottom: 0;
                    width: 3px;
                    background-image: linear-gradient(45deg, var(--gradient-start), var(--gradient-middle), var(--gradient-end));
                    transform: translateX(-50%);
                }
                .timeline-component {
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    margin: 2rem 0;
                }
                .timeline-content {
                    width: 300px;
                    text-align: left;
                    position: relative;
                }
                .timeline-circle {
                    top: -15px;
                }

                .socials a {
                    display: inline-block;
                    margin: 0 0.5rem;
                }

                .social-icon {
                    width: 3rem;
                    height: 3rem;
                    transition: transform 0.3s;
                }

                .social-icon:hover {
                    transform: scale(1.1);
                }

                @media (max-width: 768px) {
                    .skill-icons {
                        gap: 0;
                    }
                    .icon {
                        width: 5rem;
                        height: 5rem;
                    }
                    .icon-wrapper {
                        margin: 1rem;
                    }
                }
            </style>
        </head>
        <body>
            <div class="container">
                <div class="header">
                    <h1>Juraj Gijsbert van Rietbergen</h1>
                </div>
                <div class="description">
                    <p>
                        Hi, I'm Juraj. I'm a <span class="text-highlight">Software Developer</span> with experience in Rust, Java, C# and many other technologies. In my free time I enjoy tinkering with the newest technologies and trying them out in home projects.
                    </p>
                    <div class="socials">
                        <a href="mailto:jurajriet@gmail.com">
                            <img src="/static/icons/email.svg" alt="Email Icon" class="social-icon">
                        </a>
                        <a href="https://discord.com/users/107935998507237376" target="_blank">
                            <img src="/static/icons/discord.svg" alt="Discord Icon" class="social-icon">
                        </a>
                        <a href="https://www.linkedin.com/in/juraj-van-rietbergen-576585200/" target="_blank">
                            <img src="/static/icons/linkedin.svg" alt="Linekedin Icon" class="social-icon">
                        </a>
                    </div>
                </div>
                <div class="skills">
                    <h1>
                        Skills
                    </h1>
                    <div class="skill-icons">
                        <div class="icon-wrapper" data-skill="Rust, TypeScript, C#, Java, Nix">
                            <img src="/static/icons/rust.svg" alt="Rust Icon" class="icon">
                        </div>
                        <div class="icon-wrapper" data-skill="NixOS, Linux, Git, Windows">
                            <img src="/static/icons/nixos.svg" alt="NixOS Icon" class="icon">
                        </div>
                        <div class="icon-wrapper" data-skill="Docker, Podman, Libvirtd">
                            <img src="/static/icons/docker.svg" alt="Docker Icon" class="icon">
                        </div>
                    </div>
                </div>
                <div class="timeline-div">
                    <h1> 
                        Work Experience
                    </h1>
                    <div class="timeline">
                        <div class="timeline-component">
                            <div class="timeline-circle"></div>
                            <div class="timeline-content">
                                <h3>DevOps Engineer at All Tape Supplies (2023 - Present)</h3>
                                <p>Manage servers, create CI/CD pipelines, maintain infrastructure with NixOS, and support development teams in optimizing workflows and transitions to Rust applications.</p>
                            </div>
                        </div>
                        <div class="timeline-component">
                            <div class="timeline-circle"></div>
                            <div class="timeline-content">
                                <h3>Software Developer at All Tape Supplies (2021 - 2023)</h3>
                                <p>Developed logistics systems in Nuxt2 and Nuxt3, integrated SAP Service Layer, and implemented efficient solutions for warehouse management.</p>
                            </div>
                        </div>
                        <div class="timeline-component">
                            <div class="timeline-circle"></div>
                            <div class="timeline-content">
                                <h3>Software Engineer at Infoart, Zagreb (2020 - 2021)</h3>
                                <p>Worked on Java-based Administration Software for Croatian clients, assisted in building a Vue.js version, and significantly improved software performance.</p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </body>
        </html>
    "#.to_string())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/static", FileServer::from(relative!("static")))
    .mount("/", routes![index])
}