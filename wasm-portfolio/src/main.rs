use yew::prelude::*;

// ── Data ─────────────────────────────────────────────────────────────────────

struct Job {
    title: &'static str,
    company: &'static str,
    date: &'static str,
    bullets: &'static [&'static str],
}

struct Project {
    title: &'static str,
    subtitle: &'static str,
    bullets: &'static [&'static str],
}

const JOBS: &[Job] = &[
    Job {
        title: "System Engineer",
        company: "Motorola Solutions Inc.",
        date: "November 2023 – Present",
        bullets: &[
            "Architect and maintain Juniper SRX security infrastructure powering a statewide public safety communications network across multiple operational zones — protecting over 10,000 mission-critical devices relied upon by first responders.",
            "Spearheaded a county-wide firewall platform migration from Fortinet to Juniper SRX, owning the full lifecycle: policy translation, risk mitigation, cutover execution, and post-migration validation with zero service disruption.",
            "Engineered a full fiber backhaul network overhaul for a major oil and gas operator, migrating from HP to Juniper infrastructure in a zero-downtime execution across a high-stakes production environment.",
            "Designed and managed international backbone network connections supporting enterprise-scale deployments across multiple geographic regions.",
            "Built cross-platform automation pipelines in PowerShell and Bash, eliminating manual intervention for patching, configuration management, and routine maintenance across Windows and RHEL systems.",
            "Served as the decisive technical lead during a critical statewide system outage — diagnosed a production database failure, coordinated an emergency software update, and restored full operational stability under pressure.",
            "Applied defense-in-depth security principles across virtualized infrastructure spanning VMware ESXi and KVM/QEMU, hardening network segmentation and access controls across the entire stack.",
        ],
    },
    Job {
        title: "Field Engineer",
        company: "All Information Services Inc.",
        date: "April 2022 – November 2023",
        bullets: &[
            "Delivered end-to-end IT infrastructure support for SMB and municipal clients across the Chicago metro area, managing Windows and Linux environments with a focus on reliability and security.",
            "Designed, deployed, and maintained network and server infrastructure for organizations ranging from small businesses to local government — often as the sole technical resource on-site.",
            "Acted as the primary escalation point for complex infrastructure incidents, consistently resolving issues that exceeded tier-1 scope.",
        ],
    },
];

const PROJECTS: &[Project] = &[
    Project {
        title: "Statewide Public Safety Infrastructure — State of Michigan",
        subtitle: "Motorola Solutions Inc.",
        bullets: &[
            "Supported a statewide public safety radio system upgrade spanning multiple operational zones and approximately 10,000 devices.",
            "Designed and maintained custom Juniper SRX firewall configurations supporting the full statewide infrastructure footprint.",
            "Led county-wide migration from Fortinet to Juniper SRX platform including policy translation and cutover execution.",
        ],
    },
    Project {
        title: "Oil & Gas Fiber Backhaul Network Migration",
        subtitle: "Motorola Solutions Inc.",
        bullets: &[
            "Rebuilt fiber backhaul infrastructure for a major oil and gas client, migrating from HP to Juniper platform.",
            "Planned and executed the migration with zero unplanned downtime in a critical operational environment.",
            "Designed new network topology to improve redundancy and long-term maintainability.",
        ],
    },
    Project {
        title: "Personal Infrastructure & Home Lab",
        subtitle: "",
        bullets: &[
            "Designed and operate a multi-site home infrastructure environment with full VLAN segmentation and site-to-site VPN connectivity.",
            "Deployed Active Directory with centralized identity and access management across multiple network segments.",
            "Self-hosted services via Docker Compose with automated NAS backup synchronization using rsync and GNU Parallel.",
            "Applied enterprise security principles to segmentation, authentication, and service isolation throughout.",
        ],
    },
    Project {
        title: "This Website",
        subtitle: "Rust · WebAssembly · Yew",
        bullets: &[
            "Built entirely in Rust compiled to WebAssembly using the Yew framework.",
            "No JavaScript frameworks — pure Rust components rendered client-side via WASM.",
            "Bundled with Trunk and deployed as static assets to GitHub Pages.",
        ],
    },
];

// ── Components ───────────────────────────────────────────────────────────────

#[function_component]
fn Header() -> Html {
    html! {
        <header>
            <img src="andykukuc-photo.jpg" alt="Andy Kukuc professional photo" class="profile-photo" />
            <h1>{"Andy Kukuc"}</h1>
            <p class="subtitle">
                {"Network Security & Infrastructure Engineer · Critical Infrastructure · Juniper SRX"}
            </p>
            <address class="contact">
                {"Palos Hills, IL · "}
                <a href="mailto:akukuc@icloud.com">{"akukuc@icloud.com"}</a>
                {" · "}
                <a href="https://www.linkedin.com/in/andykukuc" target="_blank">{"LinkedIn"}</a>
                {" · "}
                <a href="https://github.com/andykukuc" target="_blank">{"GitHub"}</a>
            </address>
            <a class="button" href="andykukuc_resume.pdf" target="_blank">
                {"⬇ Download Resume (PDF)"}
            </a>
        </header>
    }
}

#[function_component]
fn About() -> Html {
    html! {
        <section>
            <h2>{"About Me"}</h2>
            <p>
                {"Network Security and Infrastructure Engineer specializing in critical infrastructure environments — \
                public safety, oil and gas, and government. Currently at Motorola Solutions Inc., where I design, \
                migrate, and harden enterprise network infrastructure supporting mission-critical communications systems."}
            </p>
            <p>
                {"I have hands-on experience with international backbone connectivity, fiber backhaul migrations, \
                county-wide firewall platform migrations, and Juniper SRX security policy design at scale. \
                Outside of network infrastructure, I write automation across PowerShell, Bash, Python, and Rust, \
                and have six years of experience with KVM/QEMU virtualization."}
            </p>
            <p>
                {"M.S. Cyber Forensics and Security, Illinois Institute of Technology."}
            </p>
        </section>
    }
}

#[derive(Properties, PartialEq)]
struct JobProps {
    title: &'static str,
    company: &'static str,
    date: &'static str,
    bullets: &'static [&'static str],
}

#[function_component]
fn JobEntry(props: &JobProps) -> Html {
    html! {
        <>
            <h3>{props.title}{" — "}{props.company}</h3>
            <p class="date">{props.date}</p>
            <ul>
                { for props.bullets.iter().map(|b| html! { <li>{*b}</li> }) }
            </ul>
        </>
    }
}

#[function_component]
fn Experience() -> Html {
    html! {
        <section>
            <h2>{"Experience"}</h2>
            { for JOBS.iter().map(|job| html! {
                <JobEntry
                    title={job.title}
                    company={job.company}
                    date={job.date}
                    bullets={job.bullets}
                />
            }) }
        </section>
    }
}

#[function_component]
fn Education() -> Html {
    html! {
        <section>
            <h2>{"Education"}</h2>
            <h3>{"M.S. Cyber Forensics and Security — Illinois Institute of Technology"}</h3>
            <p class="date">{"Completed"}</p>
            <h3>{"B.S. Information Technology and Management — Illinois Institute of Technology"}</h3>
            <p class="date">{"Completed"}</p>
        </section>
    }
}

#[function_component]
fn Certifications() -> Html {
    html! {
        <section>
            <h2>{"Certifications"}</h2>
            <ul>
                <li>{"CompTIA Security+"}</li>
                <li>{"CompTIA Network+"}</li>
                <li>{"Palo Alto ACE Certification"}</li>
            </ul>
        </section>
    }
}

#[function_component]
fn Skills() -> Html {
    html! {
        <section>
            <h2>{"Technical Skills"}</h2>
            <ul>
                <li><strong>{"Network & Security: "}</strong>{"Juniper SRX & EX/QFX specialist — enterprise firewall design, policy migration, and hardening across critical infrastructure; international backbone and fiber backhaul architecture; Fortinet and Palo Alto platforms"}</li>
                <li><strong>{"Virtualization: "}</strong>{"6 years hands-on KVM/QEMU expertise; VMware ESXi, Hyper-V, and Docker in production enterprise environments"}</li>
                <li><strong>{"Operating Systems: "}</strong>{"Deep proficiency across RHEL/Linux, Windows Server, and macOS — administration, hardening, and automation at scale"}</li>
                <li><strong>{"Scripting & Development: "}</strong>{"Production automation in PowerShell, Bash, and Python; systems programming in Rust; full stack web development including WebAssembly"}</li>
                <li><strong>{"Infrastructure: "}</strong>{"Enterprise identity with Active Directory; full network stack — DNS, DHCP, VPN, VLAN segmentation, and multi-site connectivity design"}</li>
                <li><strong>{"Security: "}</strong>{"Defense-in-depth architecture, firewall policy governance, incident response, and cyber forensics — backed by an M.S. in Cyber Forensics & Security"}</li>
            </ul>
        </section>
    }
}

#[derive(Properties, PartialEq)]
struct ProjectProps {
    title: &'static str,
    subtitle: &'static str,
    bullets: &'static [&'static str],
}

#[function_component]
fn ProjectCard(props: &ProjectProps) -> Html {
    html! {
        <div class="project-card">
            <h3>{props.title}</h3>
            if !props.subtitle.is_empty() {
                <p class="date">{props.subtitle}</p>
            }
            <ul>
                { for props.bullets.iter().map(|b| html! { <li>{*b}</li> }) }
            </ul>
        </div>
    }
}

#[function_component]
fn Projects() -> Html {
    html! {
        <section>
            <h2>{"Projects"}</h2>
            { for PROJECTS.iter().map(|p| html! {
                <ProjectCard
                    title={p.title}
                    subtitle={p.subtitle}
                    bullets={p.bullets}
                />
            }) }
        </section>
    }
}

#[function_component]
fn Footer() -> Html {
    html! {
        <footer>
            <p>
                {"© Andy Kukuc — "}
                <a href="https://github.com/andykukuc" target="_blank">{"GitHub"}</a>
            </p>
            <p style="color:#8ca0b3; font-size:0.95em; margin-top:8px;">
                {"Built with Rust + WebAssembly · Yew · GitHub Pages"}
            </p>
        </footer>
    }
}

// ── App ──────────────────────────────────────────────────────────────────────

#[function_component]
fn App() -> Html {
    html! {
        <>
            <Header />
            <main>
                <About />
                <hr />
                <Experience />
                <hr />
                <Education />
                <hr />
                <Certifications />
                <hr />
                <Skills />
                <hr />
                <Projects />
            </main>
            <Footer />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
