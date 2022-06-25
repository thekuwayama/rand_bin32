// import
import init, {rand_bin32_base64, rand_bin32_hex} from '../../pkg/rand_bin32.js'
import '../css/style.css'

// function
export function base64() {
    init()
        .then(() => {
            if (document.getElementById('output').innerHTML.length > 0 && !confirm('再生成しますか？')) {
                return
            }
            
            var s
            try {
                s = rand_bin32_base64()
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
            if (document.getElementById('output').innerHTML.length > 0 && !confirm('再生成しますか？')) {
                return
            }

            var s
            try {
                s = rand_bin32_hex()
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
