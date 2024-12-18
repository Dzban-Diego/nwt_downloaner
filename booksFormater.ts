import { books } from "./books";
import type { Book } from "./type";

function main() {
	const newBooks: Book[] = [];

	for (const [_, value] of Object.entries(books)) {
		newBooks.push({
			chapterCount: Number.parseInt(value.chapterCount),
			title: value.standardName,
		});
	}
	console.log(JSON.stringify(newBooks));
}

main();
