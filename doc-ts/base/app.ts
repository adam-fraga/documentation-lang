// Pour compiler le TS on utilise "npx tsc "file.ts""
// Avec dossier src et dist npx tsc src/app.ts --outDir dist
// Le fichier tsconfig.json automatise la ligne ci dessus (ajouté --watch)

const a: string = "Hello world"
const n: number = 23 
const b: boolean = true
const d: null = null

const arrStr: string[] = ["Tableau", "de", "string"]
const arrMix: any[] = ["Tableau", "mixte", 23, true]

const user: {firstname: string, lastname: string} = {firstname: "John", lastname: "Doe"}
const userOptionnal: {firstname?: string, lastname?: string} = {}
//Objet avec une infinité de clés
const infiniteKey: {[key: string]: string} =
  {
    firstname: "Adam", 
    lastname: "Frg", 
    color: "blue"
  }

const date: Date = new Date()
const callBack: (e: MouseEvent) => void = (e: MouseEvent): number => {
  return 3
}
const printID: Function = (id: number): number => {
  console.log(id.toString());
  return id
}
// Le mot clés as permet de force le typage d'une variable
const count = document.querySelector('#count') as HTMLButtonElement
// Même comportement avec syntaxe différente
const count2 = <HTMLButtonElement>document.querySelector('#count')

// Union de type
const printMix: Function = (id: number): number | string=> {
  console.log(id.toString());
  return id
}
