import { convert } from 'html-to-text';
import {books}  from './books.js'
import fs from 'fs'
import { promisify } from 'util'
const appendFileAsync = promisify(fs.appendFile)
const writeFileAsync = promisify(fs.writeFile)

function addLeadingZeros(num, totalLength) {
    return String(num).padStart(totalLength, '0');
}

const getChapter = async (book, chapter) => {
    console.log(`Pobieranie rozdziału ${book} ${chapter}`)
    const chapterID = `${book}${addLeadingZeros(chapter, 3)}`
    const range = `${chapterID}001-${chapterID}179`
    const url = `https://www.jw.org/pl/biblioteka/biblia/biblia-wydanie-do-studium/ksiegi/json/html/${range}`
    const response = await fetch(url)
    const data = await response.json()
    return data.ranges[range].verses.map(verse => {
        let text = verse.content.replaceAll('href', 'test')
        text = text.replaceAll(' ', ' ')
        text = convert(text, {
            wordwrap: null
        })
        return {
            ...verse,
            content: text,
            url: `https://jw.org/finder?srcid=jwlshare&wtlocale=P&prefer=lang&bible=${verse.vsID}`
        }
    })


}

const getBook = async (bookID) => {
    const book = books[bookID]
    console.log(`Pobieranie księgi: ${book.standardName}`)
    const chapters = {}
    for(let i = 1; i <= book.chapterCount; i++){
        chapters[i] = await getChapter(bookID, i)
    }
    const data = {...book, chapters}
    return data
}
const main = async () => {
    await writeFileAsync('booksWithContent.js', 'export const books = {\n', function (err, file) {
        if (err) throw err;
        console.log(`Stworzono plik`);
    })
    for(let i = 1; i <= 66; i++){
        const book = await getBook(i)
        await appendFileAsync('booksWithContent.js', `${i}: ${JSON.stringify(book)}, \n`).then(() => {
            console.log('zapisano')
        }).catch(console.error)
    }
    appendFileAsync('booksWithContent.js', '}')
}

main()
