let person1 = {
  picture: { "value": "img/foto.jpg", "settings": { "type": "img" } },
  name: { "value": "Pablo Centurion", "settings": { "type": "str" } },
  dni: { "value": "99.999.999", "settings": { "type": "str" } },
  dose: { "value": .54, "settings": { "type": "str" } },
};

let person2 = {
  picture: { "value": "img/foto.jpg", "settings": { "type": "img" } },
  name: { "value": "Pablo Centurion", "settings": { "type": "str" } },
  dni: { "value": "99.999.999", "settings": { "type": "str" } },
  dose: { "value": .54, "settings": { "type": "str" } },
};



const operators_data =
{
  headers: [{ title: "" }, { title: "Name" }, { title: "DNI" }, { title: "Dose" }],
  rows: [person1, person2],
};


export default operators_data