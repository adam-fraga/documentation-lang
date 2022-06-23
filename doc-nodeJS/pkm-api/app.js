const express = require('express') //Ajout du packet express
const { success } = require('./helper.js') //Import de la fonction success
let pokemons = require('./mock-pokemon')
const app = express() //Instance app express
const port = 3000

//Définition du Middleware
const logger = (req, res, next) => {
  console.log(`URL: ${req.url}`)
  next() //indique la fin du traitement du middleware
}
//Utilisation du middleware (il est possible de supprimer la définition pour la passé
//directement en parametre)
app.use(logger)

app.get("/", (req, res) => res.send("Hello")) //Methode http (get), chemin, methode gestionnaire

app.get("/api/pokemon/:id", (req, res) => {
  const id = parseInt(req.params.id) //Express renvoi par defaut les param en string
  const pokemon = pokemons.find(pokemon => pokemon.id === id)
  //Convertie la réponse au format json et utilise le helper pour formater la réponse
  res.json(helper.success(message, pokemon))
})

app.listen(port, () => console.log(`Notre application Node est démarré sur : http://localhost:${port}`))
