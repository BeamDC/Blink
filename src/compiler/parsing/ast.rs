/* Blink Language Grammar:
 */

/*
 * # *** TOP LEVEL *** #
 *
 * Param:
 *   $Ident $Colon @TypeExpr
 *
 * ParamList:
 *   (@Param $Comma)* @Param?
 *
 * FnSignature:
 *   $Fn $Ident @ParamList $Arrow @TypeExpr
 *
 * FnDef:
 *   @FnSignature @Block
 *
 * # *** BLOCK LEVEL *** #
 *
 * Block:
 *   { @Statement* }
 *
 */

/*
 * # *** EXPR LEVEL *** #
 *
 * Literal:
 *   $Numeric
 *   | $String
 *   | $Char
 *   | $True
 *   | $False
 *
 */

 /*
 * # *** OP LEVEL *** #
 *
 */
