/**
  @author Nabil Redmann <repo+gist@bananaacid.de>
  @license MIT
  @version 1.3
  @usage
    simple param parsing
    --param1             =>  argv.param1 = true
    --param2 "ab c"      =>  argv.param2 = 'ab c'
    --param3 abc         =>  argv.param3 = 'abc'
    -p4                  =>  argv.p4 = true
    -p4 val              =>  argv.p4 = 'val'
    -p4 "val 2"          =>  argv.p4 = 'val 2'
    /p5                  =>  argv.p5 = true
    /p5 val              =>  argv.p5 = 'val'
    /p5 "val 2"          =>  argv.p5 = 'val 2'
    --param6="abc = def" =>  argv.param6 = 'abc = def'
    /p7="abc = def"      =>  argv.p7 = 'abc = def'

  @note
    additional arguments after `--` will be stored as an array and not be touched,
    parsing of these can be done by using the exported `parse(args)` function

  @example
    index.mjs
        import args, { rest, dashed, parse } from './params-extended-dashed.mjs';
        console.log(process.argv, args);
        console.log('rest ones:', rest);
        console.log('after --:', dashed);
    $ node index.mjs --param1 a1 --param2="asdad =addd" /p as asdadd -- --safsdfasdf 123 fffff
**/

let parseFn = (accumulator, currentValue, currentIndex, source) => {
  if (currentValue && !!~['-', '/'].indexOf(currentValue[0])) {
      let cv = currentValue.replace(/^(-|\/)+/, '');
      let val = true;
      if (!!~cv.indexOf('=')) {
          let [cvNew, ...rest] = cv.split('=');
          cv = cvNew;
          val = rest.join('=');
      } else if (source[currentIndex + 1] && (!~['-', '/'].indexOf(source[currentIndex + 1][0]))) {
          val = source[currentIndex + 1];
          source[currentIndex + 1] = undefined;
      }
      accumulator.params[cv] = val;
  } else if (currentValue) {
      // left over value
      //console.error('value can not be associated:', currentValue);
      accumulator.rest.push(currentValue);
  }
  return accumulator;
};


export const parse = argsBase => {
  const raw = argsBase.join(' ')
  const idx = argsBase.indexOf('--');
  let argsDashed = [];
  if (!!~idx) {
    argsDashed = argsBase.splice(idx);
    argsDashed.shift();
  }

  const args = argsBase.reduce(parseFn, { 'params': {}, 'rest': [] });
  args.params._raw = raw
  args.params._ = args.rest
  return args
}
