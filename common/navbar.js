var width_query = window.matchMedia("(max-width: 600px)");

width_query.addEventListener("change", function() {
    document.getElementById("nav-links").style.display = width_query.matches ? "none" : "block";
})

function nav_toggle_links() {
    var links = document.getElementById("nav-links");
    if (width_query.matches) {
        if (links.style.display == "block") {
            links.style.display = "none";
        } else {
            links.style.display = "block";
        }
    }
}

const NAVBAR_LINKS = {
    home: "/",
    dev: "/dev/",
    cloud: "https://cloud.rbuurman.de",
    misc: "/misc/",
}

class NavBar extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        let wrapper = document.createElement("nav");
        wrapper.id = "nav-bar";

        let head = document.createElement("div");
        head.id = "nav-head";

        let homepageLink = document.createElement("a");
        homepageLink.href = "/";

        let homepageIcon = document.createElement("img");
        homepageIcon.id = "nav-icon";
        homepageIcon.src = "/assets/logo.png";

        let label = document.createElement("p");
        label.id = "nav-label";
        if (this.hasAttribute("label")) {
            label.textContent = this.getAttribute("label");
        }

        let hamburger = document.createElement("img");
        hamburger.id = "nav-hamburger";
        hamburger.src = "/assets/hamburger.svg";
        hamburger.onclick = () => {nav_toggle_links()};

        let links = document.createElement("ul");
        links.id = "nav-links";

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

            entry.appendChild(link);
            links.appendChild(entry);
        }

        homepageLink.appendChild(homepageIcon);
        head.appendChild(homepageLink);
        head.append(label);
        head.append(hamburger);

        wrapper.appendChild(head);
        wrapper.appendChild(links);
        this.appendChild(wrapper);
    }
}

customElements.define("nav-bar", NavBar);
