<?php

require __DIR__.'/vendor/autoload.php';

use Symfony\Component\ExpressionLanguage\ExpressionLanguage;

echo "Expression Language: ";
$expressionLanguage = new ExpressionLanguage();
$s = microtime(true);
echo $expressionLanguage->evaluate("((5 + 4) / 3) * 3 * (100 / 20 ) - 3");
printf(" Duration: %.6fs\n\n", microtime(true) - $s);

echo "Rust FFI: ";
$s = microtime(true);
$ffi = FFI::cdef(<<<EOH
const char *eval_expression(const char *expr);
EOH, __DIR__.'/../target/debug/libexpression_language.so');
echo $ffi->eval_expression("((5 + 4) / 3) * 3 * (100 / 20 ) - 3");
printf(" Duration: %.6fs\n\n", microtime(true) - $s);

echo "PHP eval: ";
$s = microtime(true);
echo (((5 + 4) / 3) * 3 * (100 / 20 ) - 3);
printf(" Duration: %.6fs\n\n", microtime(true) - $s);
