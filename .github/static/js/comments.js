
class Comments {

    static service = "https://icanhazdadjoke.com/";
    static containerId = "comments";

    static start(parent) {
        let comments = new Comments(parent);
        window.addEventListener("load", ev => {
            comments.load();
        });
    }

    constructor(parent) {
        if (false) {
            let script = document.createElement("script");
            script.type = "text/javascript";
            // script.src = "https://raw.githubusercontent.com/jpedro/js/master/v1/test.js";
            // script.src = "https://js.jpedro.dev/test.js";
            // script.src = "https://jpedro.github.io/js/v1/test.js";
            script.src = "https://raw.githubusercontent.com/jpedro/jpedro.github.io/master/.github/static/js/comments.js";
            document.head.appendChild(script);
        }

        console.log("parent", parent);
        parent ||= document.body;
        console.log("parent", parent);
        this.container = this.create(parent);
    }

    create(parent) {
        let div = document.createElement("div");
        div.id = Comments.containerId;
        parent.appendChild(div);
        return div;
    }

    load() {
        const total = Math.random() * (5 - 1) + 1;
        const title = document.createElement("h2");
        const ul = document.createElement("ul");

        title.innerText = "Expert comments";
        this.container.appendChild(title);
        this.container.appendChild(ul);

        for (let i = 0; i < total; i++) {
            this.get()
                .then(data => {
                    const li = document.createElement("li");
                    li.innerHTML = data.joke;
                    ul.appendChild(li);
                })
                .catch(err => {
                    console.error("err", err);
                })
            ;
        }
    }

    async get() {
        const options = {
            headers: {
                "Accept": "application/json",
            },
        };

        const res = await fetch(Comments.service, options)
        if (res.ok) {
            const data = await res.json();
            return Promise.resolve(data);
        }

        return Promise.reject("Failed to load VIP comment");
    }

}
