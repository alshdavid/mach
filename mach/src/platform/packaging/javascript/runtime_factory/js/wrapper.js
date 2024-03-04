(async () => {
  // Code goes here

  // adds loaded state to currentScript to avoid races with async bundles
  if (document.currentScript) document.currentScript.loaded = 1;
})()
