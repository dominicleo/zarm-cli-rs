import { bundless } from 'zarm-cli';

bundless({
  format: "esm",
  root: '../library',
  input: 'src_zarm',
  output: 'dist/esm',
  ignores: [
    '**/*.{md,mdx}',
    '**/*.d.{ts,tsx}',
    '**/__{test,tests}__/**',
    '**/*.{test,e2e,spec}.{js,jsx,ts,tsx}',
  ],
});
