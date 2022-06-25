// import
import init, {random_bin32_base64, random_bin32_hex} from '../../pkg/rand_bin32.js'
import '../css/style.css'

// function
export function base64() {
    init()
        .then(() => {
            var s
            try {
                s = random_bin32_base64()
            } catch(e) {
                alert(e)
                return
            }

            document.getElementById('output').innerHTML = s
        })
}

export function hex() {
    init()
        .then(() => {
            var s
            try {
                s = random_bin32_hex()
            } catch(e) {
                alert(e)
                return
            }

            document.getElementById('output').innerHTML = s
        })
}

// init
document.getElementById('base64').onclick = base64
document.getElementById('hex').onclick = hex
