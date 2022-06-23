-- Les types de variable en lua

type(42) -- Le type est `number`
type("Tutoriel Lua") -- Le type est `string`
type(false) -- Le type est `boolean`
type(var) -- Le type est `nil`, puisque `var` n’est pas défini

--IMPORTANT LES TABLEAUX EN LAU DEMARRE A L'INDICE 1 ET NON PAS 0

-- Opération arithmétique dans Lua
local addition = 1 + 2 -- évalué en tant que la valeur `3`
print(addition)

-- Concaténation de chaînes de caractères dans Lua
local str ='Walther' .. 'White' -- évalué comme `WaltherWhite`
print(str)

-- Concaténation de chaînes de caractères avec conversion automatique d’un chiffre dans Lua
local str2 = 'Les ' .. 3 .. ' mousquetaires' -- évalué comme `Les 3 mousquetaires`
print(str2)

-- Test d’équivalence dans Lua
7 == '7' -- évaluer comme `false`

-- Test d’équivalence dans Lua
'petit' == string.lower('PETIT') -- évalué comme `true`

-- information de type dynamique
type(var) == 'nil' -- évalué comme `true`, puisque `var` n’est pas défini

-- En lua le signe différent de est ~= et non pas !=

Prix = 42,99
Remise = 0,15 -- 15 % de remise
Prix -= Prix * Remise -- `-=` ne fonctionne pas dans Lua
-- La décrémentation doit être écrite de façon explicite
Prix = Prix - (Prix * Remise)

-- portée externe
do
  local x = 1
  do -- portée interne
    local y = 2
    -- générer `z` dans la portée globale
    -- dans ce cadre, accès à la variable locale `x` de la portée externe
    -- et variable locale `y` de la portée interne
    Z = x + y -- `z` a maintenant la valeur `3`
  end
  print(x) -- donne `1`
  print(y) -- donne `nil` puisque `y` n’existe pas dans la portée externe
  print(Z) -- donne `3`
end
-- `z` est global et existe également en dehors de la portée externe
Z = Z + 4
print(Z) -- donne `7`


-- Structure de controles
local limit = 42;
local nombre = 43;
if nombre < limit then
  print("En-dessous de la limite.")
elseif nombre == limit then
  print("Précisément à la limite…")
else
  print("Au-dessus de la limite !")
end

-- Les boucles

limit = 10
nombre = 1
while nombre <= limit do
  print("nombre:", nombre)
  nombre = nombre + 1
end
-- Attention : bien que `nombre` soit déjà supérieur à `limit`,
-- le corps de la boucle est exécuté une fois
nombre = 11
repeat
  print("nombre:", nombre)
  nombre = nombre + 1
until nombre > limit


-- Lua déclare implicitement local dans la boucle for contrairement à JS
local debut = 1
local fin = 10
for nombre = debut, fin do
  print("Nombre actuel :", nombre) -- `1,2,3,4,5,6,7,8,9,10`
end
-- définir explicitement l’etape sur `2`
local etape = 2
for nombre = debut, fin, etape do
  print("Nombre actuel :", nombre) -- `1,3,5,7,9`
end
-- l’etape peut être négative
etape = -2
-- permuter le debut et la fin avec une etape négative, pour compter de façon décroissante
for nombre = fin, debut, etape do
  print("Nombre actuel :", nombre) -- `10,8,6,4,2`
end

-- Définir une liste d’annees
local decennies = {1910, 1920, 1930, 1940, 1950, 1960, 1970, 1980, 1990}
-- accéder aux annees individuelles à l’aide d’un itérateur
for index,annee in ipairs(decennies) do
  print(index,annee)
end

-- Les fonctions

-- Définir la procédure
function Bonjour(nom)
  print("Bonjour", nom)
end
-- Appeler la fonction
Bonjour("très cher monsieur")
-- l’écriture suivante est également possible
Bonjour "très cher monsieur"
-- toutefois, la syntaxe suivante ne fonctionne pas
nom = "Walther"
Bonjour nom -- erreur de syntaxe
-- avec une variable à la place d’un littéral, la fonction doit être appelée avec des parenthèses
Bonjour(nom)

-- imprimer tous les arguments d’une fonction
function Var_args(...)
  for index, arg in ipairs({...}) do
    print(index, arg)
  end
end
Var_args('Peter', 42, true)

-- Fonction avec plusieurs valeurs de retour
local function premier_et_dernier(liste)
  -- fournit le premier et le dernier élément de la liste
  -- chaque valeur de retour est séparée par une virgule `,`
  return liste[1], liste[#liste]
end

local personnes = {"Jim", "Jack", "John"}
-- Attribution des valeurs de retour à plusieurs variables
local premier, dernier = premier_et_dernier(personnes)
print("Le premier est", premier)
print("Le dernier est", dernier)

local function min_moyenne_max(...)
  -- définir les valeurs de début pour `min` et `max` sur le premier argument
  local min = select(1, ...)
  local max = select(1, ...)
  -- Définir la valeur médiane sur zéro au début
  local moyenne = 0
  -- itérer sur les chiffres
  -- nous n’avons pas besoin de la variable index
  -- nous utilisons `_` en tant que balise
  for _, nombre in ipairs({...}) do
    -- définir un nouveau minimum le cas échéant
    if min > nombre then
      min = nombre
    end
    -- définir un nouveau maximum le cas échéant
    if max < nombre then
      max = nombre
    end
    -- additionner des chiffres pour la moyenne
    moyenne = moyenne + nombre
  end
  -- diviser la somme des nombres par leur nombre
  moyenne = moyenne / #{...}
  return min, moyenne, max
end
-- ici, nous n’avons pas besoin de la valeur `moyenne`
-- nous utilisons `_` en tant que balise
min, _, max = min_moyenne_max(78, 34, 91, 7, 28)
print("Le minimum et le maximum des nombres sont", min, max)

-- fonction `map()` en Lua
-- reçoit une fonction `f` et une liste comme arguments
local function map(f, liste)
  -- créer une nouvelle liste pour les valeurs de sortie
  local _liste = {}
  -- itérer sur les éléments de la liste avec index
  for index, valeur in ipairs(liste) do
    -- utiliser la fonction `f()` sur la valeur actuelle de la liste
    -- et enregistrer la valeur de retour dans la nouvelle liste sur le même index
    _liste[index] = f(valeur)
  end
  -- retourner une nouvelle liste
  return _liste
end
-- Liste de chiffres
local nombres = {3, 4, 5}
-- Fonction appliquée à tous les éléments de la liste
local function carre(nombre)
  return nombre * nombre
end
-- génération des carrees via la fonction `map()`
local carrees = map(carre, nombres) -- `{9, 16, 25}`
-- obtenir les carrees
for _, nombre in ipairs(carrees) do
  print(nombre)
end


