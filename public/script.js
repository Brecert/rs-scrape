import basicCard from './components/basic-card.js'
import {render, html} from '//cdn.pika.dev/lighterhtml';

const defaultUrl = () => (new URLSearchParams(window.location.search)).get('url') || 'https://bree.ml/'

async function getMeta(url = defaultUrl()) {
	return await fetch(`${window.location.origin}/api?url=${url}`)
		.then(res => res.json())
		.catch(err => console.error(err))
}

const main = async () => {
	const App = (meta) => html`
		${basicCard(meta)}
	`

	let meta = await getMeta(defaultUrl())
	const renderApp = () => render(document.body, () => App(meta))

	renderApp()
}

main()