
class Comments {

    static service = "https://icanhazdadjoke.com/";
    static containerId = "comments";

    constructor(mount) {
        let script = document.createElement("script");
        script.type = "text/javascript";
        // script.src = "https://raw.githubusercontent.com/jpedro/js/master/v1/test.js";
        script.src = "https://js.jpedro.dev/test.js";
        document.head.appendChild(script);
        console.log("mount", mount);
        mount ||= document.body;
        console.log("mount", mount);
        this.container = this.createContainer(mount);
    }

    createContainer(mount) {
        let div = document.createElement("div");
        div.id = Comments.containerId;
        mount.appendChild(div);
        return div;
    }

    async getComment() {
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

    load() {
        const total = Math.random() * (5 - 1) + 1;
        const title = document.createElement("h2");
        const ul = document.createElement("ul");

        title.innerText = "Expert comments";
        this.container.appendChild(title);
        this.container.appendChild(ul);

        for (let i = 0; i < total; i++) {
            this.getComment()
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

    static start(mount) {
        let comments = new Comments(mount);
        window.addEventListener("load", ev => {
            comments.load();
        });
    }

}
