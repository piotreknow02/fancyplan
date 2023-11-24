// Constants
const table = document.querySelector("table.tabela")
const days = document.querySelectorAll("th")
const hours = [...document.querySelectorAll("td.g")]
    .map(e => e.innerText)
    .map(e => {
        const [start, end] = e.split("-")
        return {start: start.trim(), end: end.trim()}
    })

/**
 * Puts timer on the lesson plan
 */
function updateTimer() {
    const col = chooseColumn()
    const {row, isBreak, time} = chooseHour()
    
    if (row == null) return

    const currentCell = table.children[0].children[row].children[col]

    currentCell.classList.add("active")
    if (isBreak) {
        currentCell.classList.add("break")
    }else {
        currentCell.classList.remove("break")
    }

    let timer = currentCell.querySelector("div.timer")
    if (timer == null) {
        timer = document.createElement("div")
        timer.classList.add("timer")
        currentCell.appendChild(timer)
    }
    timer.innerText = (isBreak ? "Lekcja za" : "Koniec lekcji za") + ` ${time.min} min ${time.sec} s`

    if (row > 0) {
        const lastCell = table.children[0].children[row-1].children[col]
        cleanup(lastCell)
    }
}
    
/**
 * Gets collumn number for current day
 * @returns {number} column number
*/
function chooseColumn() {
    const {format} = new Intl.DateTimeFormat('pl', {weekday: 'long'});
    let day = format()
    day = day.charAt(0).toUpperCase() + day.substring(1)

    for(const d in days) {
        if (days[d].innerText.trim() === day) {
            return d
        }
    }
    throw new Error("error getting column")
}

/**
 * Gets row number for current hour
 * @returns {object} returns object that contains row, isBreak and time to end of lesson or break
 */
function chooseHour() {
    const currentTime = new Date()
    for (const t in hours) {
        const startTime = readDate(hours[t].start)
        const endTime = readDate(hours[t].end)

        if (startTime === undefined || endTime === undefined) {
            continue
        }

        if (currentTime > endTime) {
           continue
        }
        else if (currentTime > startTime) {
            return {row: parseInt(t) + 1, isBreak: false, time: calculateTime(endTime)}
        }
        else {
            return {row: parseInt(t) + 1, isBreak: true, time: calculateTime(startTime)}
        }
    }
    return {row: null, isBreak: false, time: {min: 0, sec: 0}}
}

/**
 * Calculates time to end of break or lesson
 * @param {string} time time string to count from
 * @returns {string} time string
 */
function calculateTime(lessonTime) {
    const currentTime = new Date()
    const timeLeft = Math.abs((lessonTime-currentTime)/1000/60)
    const seconds = (timeLeft % 1) * 60

    return {min: Math.floor(timeLeft), sec: Math.round(seconds)}
}

/**
 * Converts hour to date
 * @param {string} hour
 * @returns {Date} this hour today
 */
function readDate(hour) {
    const d = new Date()
    const hms = hour.split(":")
    d.setHours(hms[0])
    d.setMinutes(hms[1])
    d.setSeconds(hms[2] ?? 0)
    return d
}

/**
 * Cleans passed html tag from timer and color
 * @param {object} elem html td tag to clean 
 */
function cleanup(elem) {
    elem.classList.remove("active")
    const timer = elem.querySelector("div.timer")
    if (timer != null)
        document.removeChild(timer)
}

// Main call
setInterval(updateTimer, 1000)
