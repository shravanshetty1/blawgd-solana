
function captcha(response) {
    let data = {
        address: JSON.parse(localStorage.getItem("app_data")).address,
        captcha: response,
    }
    let dst = location.protocol+"//faucet."+location.hostname+":"+location.port
    console.log(data)
    console.log(dst)
    fetch(dst, {
        method: "POST",
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(data)
    }).then(res => {
        if (res.status!==200) {
            console.log(res.status)
        } else {
            window.location.href = window.location.protocol+"//"+window.location.host
        }
    });
}

function wasm_progress(loaded, total) {
    let percentage = (loaded/total) * 100;
    if (percentage < 100) {
        document.getElementById("progress").innerHTML = "Loaded "+percentage.toFixed(1)+"%"
    } else {
        document.getElementById("progress").innerHTML = "Rendering..."
    }
}

async function wasm_progress_handler(url) {
    // Get your normal fetch response
    let response = await fetch(url);

    let total = 2621440;
    let loaded = 0;

    let res = new Response(new ReadableStream({
        async start(controller) {
            let reader = response.body.getReader();
            for (; ;) {
                let {done, value} = await reader.read();

                if (done) {
                    wasm_progress(total, total)
                    break
                }

                loaded += value.byteLength;
                wasm_progress(loaded, total)
                controller.enqueue(value);
            }
            controller.close();
        },
    }, {
        "status": response.status,
        "statusText": response.statusText
    }));

// Make sure to copy the headers!
// Wasm is very picky with it's headers and it will fail to compile if they are not
// specified correctly.
    for (let pair of response.headers.entries()) {
        res.headers.set(pair[0], pair[1]);
    }

    return res;
}