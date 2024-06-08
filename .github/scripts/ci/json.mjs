import fs from 'node:fs'
import path from 'node:path'

let cwd = process.cwd();
let [,,target, key, value] = process.argv

if (!path.isAbsolute(target)) {
  target = path.join(cwd, target)
}

function set(obj, keys, val) {
	keys.split && (keys=keys.split('.'));
	var i=0, l=keys.length, t=obj, x, k;
	while (i < l) {
		k = keys[i++];
		if (k === '__proto__' || k === 'constructor' || k === 'prototype') break;
		t = t[k] = (i === l) ? val : (typeof(x=t[k])===typeof(keys)) ? x : (keys[i]*0 !== 0 || !!~(''+keys[i]).indexOf('.')) ? {} : [];
	}
}

const get = (obj, path, defaultValue = undefined) => {
  const travel = regexp =>
    String.prototype.split
      .call(path, regexp)
      .filter(Boolean)
      .reduce((res, key) => (res !== null && res !== undefined ? res[key] : res), obj);
  const result = travel(/[,[\]]+?/) || travel(/[,[\].]+?/);
  return result === undefined || result === obj ? defaultValue : result;
};

const original = JSON.parse(fs.readFileSync(target, 'utf8'))

if (value !== undefined) {
  set(original, key, value)
  fs.writeFileSync(target, JSON.stringify(original, null, 2), 'utf8')
} else {
  process.stdout.write(get(original, key))
}
