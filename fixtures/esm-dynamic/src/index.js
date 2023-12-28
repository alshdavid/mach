async function main() {
	const a_module = await import('./a.js')
	console.log(a_module.a)
}

main()