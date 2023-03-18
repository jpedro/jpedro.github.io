const MAX = 5;
const MIN = 1;

const getComment = async (content, callback) => {
    const options = {
        headers: {
            "Accept": "application/json",
        },
    };

    const res = await fetch("https://icanhazdadjoke.com/", options)
    if (res.ok) {
        const data = await res.json();
        return Promise.resolve(data);
    }

    return Promise.reject("Failed to load VIP comment");
};

const loadComments = () => {
    const total = Math.random() * (MAX - MIN) + MIN;
    const div = document.createElement("div")
    const h4 = document.createElement("h2")
    const ul = document.createElement("ul")
    div.id = "comments"
    h4.innerText = "Expert comments";
    div.appendChild(h4);
    div.appendChild(ul);
    document.body.children[0].appendChild(div);

    for (i = 0; i < total; i++) {
        getComment()
        .then(data => {
            const li = document.createElement("li");
            li.innerHTML = data.joke;
            ul.appendChild(li);
        })
        .catch(err => {
            console.error("err", err);
        });
    }
};

window.addEventListener("load", (event) => {
    // document.body.style.backgroundColor = "#ffc";
    loadComments();
});
