import {books} from './booksWithContent.js'
const book = process.argv[2]
const chapter = process.argv[3]
const verse = parseInt(process.argv[4]) - 1
console.log(books[book].chapters[chapter][verse])
