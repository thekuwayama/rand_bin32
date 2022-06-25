// import
import init, {rand_bin32_base64, rand_bin32_hex} from '../../pkg/rand_bin32.js'
import '../css/style.css'

// function
function rand(f) {
    init()
        .then(() => {
            if (document.getElementById('output').innerHTML.length > 0 && !confirm('再生成しますか？')) {
                return
            }
            
            var s
            try {
                s = f()
            } catch(e) {
                alert(e)
                return
            }

            document.getElementById('output').innerHTML = s
        })
}

export function base64() {
    rand(rand_bin32_base64)
}

export function hex() {
    rand(rand_bin32_hex)
}

// init
document.getElementById('base64').onclick = base64
document.getElementById('hex').onclick = hex
