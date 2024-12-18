import fs from "node:fs";
import { promisify } from "node:util";
import { convert } from "html-to-text";
import books from "./books.json";
import type { UrlResonse, Verse } from "./type";
const appendFileAsync = promisify(fs.appendFile);
const writeFileAsync = promisify(fs.writeFile);

function addLeadingZeros(num: number, totalLength: number) {
	return String(num).padStart(totalLength, "0");
}

const FILE_NAME = "booksWithContent.json";

const getChapter = async (
	bookId: number,
	chapterId: number,
): Promise<Verse[]> => {
	console.log(`Pobieranie rozdziału ${bookId} ${chapterId}`);
	const chapterIdRagne = `${bookId}${addLeadingZeros(chapterId, 3)}`;

	const range = `${chapterIdRagne}001-${chapterIdRagne}179`;
	const url = `https://www.jw.org/pl/biblioteka/biblia/biblia-wydanie-do-studium/ksiegi/json/html/${range}`;
	const response = await fetch(url);
	const data = (await response.json()) as UrlResonse;
	return data.ranges[range].verses.map((verse, idx) => {
		let text = verse.content;
		text = convert(text, {
			wordwrap: null,
		});

		const textContent = text.split("] ")[1];

		return {
			verseId: idx + 1,
			chapterId: chapterId,
			bookId: chapterId,
			content: textContent,
			url: `https://jw.org/finder?bible=${verse.vsID}`,
		};
	});
};

const getBook = async (bookId: number) => {
	const book = books[bookId - 1];
	console.log(`Pobieranie księgi: ${book.title}`);
	const chapters: Verse[][] = [];
	for (let i = 1; i <= book.chapterCount; i++) {
		chapters.push(await getChapter(bookId, i));
	}
	const data = { ...book, bookId: bookId, chapters };
	return data;
};

const main = async () => {
	await writeFileAsync(FILE_NAME, "[\n");
	for (let i = 1; i <= 66; i++) {
		const book = await getBook(i);
		await appendFileAsync(FILE_NAME, `${JSON.stringify(book, null, 2)}, \n`)
			.then(() => {
				console.log("Zapisano");
			})
			.catch(console.error);
	}
	appendFileAsync(FILE_NAME, "]");
};

main();
