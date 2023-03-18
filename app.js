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
    const h4 = document.createElement("h4")
    const ul = document.createElement("ul")
    h4.innerText = "Expert comments";
    document.body.appendChild(h4);
    document.body.appendChild(ul);

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
    document.body.style.backgroundColor = "#ffc";
    loadComments();
});
