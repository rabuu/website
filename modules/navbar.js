const nav_width_query = window.matchMedia("(max-width: 600px)");

const nav_width_query_listener = () => {
    document.getElementById("nav-links").style.display =
        nav_width_query.matches ? "none" : "block";
}

nav_width_query.addEventListener("change", nav_width_query_listener);

function nav_toggle_links() {
    var links = document.getElementById("nav-links");
    if (nav_width_query.matches) {
        if (links.style.display == "block") {
            links.style.display = "none";
        } else {
            links.style.display = "block";
        }
    }
}

const NAVBAR_LINKS = {
    homepage: "/",
    projekte: "/projekte/",
    krimskrams: "/krimskrams/",
    cloud: "https://cloud.rbuurman.de",
}

class NavBar extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        let wrapper = document.createElement("div");
        wrapper.id = "nav-bar";

        let head = document.createElement("div");
        head.id = "nav-head";

        let homelink = document.createElement("a");
        homelink.href = "/";

        let logo = document.createElement("img");
        logo.id = "nav-icon";
        logo.src = "/assets/logo.png";

        homelink.append(logo);

        let label = document.createElement("h1");
        label.id = "nav-label";
        if (this.hasAttribute("label")) {
            label.textContent = this.getAttribute("label");
        }

        let hamburger = document.createElement("img");
        hamburger.id = "nav-hamburger";
        hamburger.src = "/assets/hamburger.svg";
        hamburger.onclick = () => {nav_toggle_links()};

        head.append(homelink);
        head.append(label);
        head.append(hamburger);

        let links = document.createElement("ul");
        links.id = "nav-links";
        links.role = "navigation";

        for (let [name, href] of Object.entries(NAVBAR_LINKS)) {
            let entry = document.createElement("li");
            entry.id = `nav-link-${name}`;

            if (this.hasAttribute("active")) {
                let active = this.getAttribute("active");
                if (active === name) {
                    entry.classList.add("active");
                }
            }

            let link = document.createElement("a");
            link.href = href;
            link.textContent = name;

            entry.append(link);
            links.append(entry);
        }

        wrapper.append(head);
        wrapper.append(links);

        this.append(wrapper);
    }
}

export default NavBar;
