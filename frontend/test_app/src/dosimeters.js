let dosimeter1 = {
  make: { "value": "Marca X", "settings": { "type": "str" } },
  model: { "value": "Modelo Y", "settings": { "type": "str" } },
  sn: { "value": "999999999999", "settings": { "type": "str" } },
  calibration: { "value": "12/01/2019", "settings": { "type": "str" } },
};

let dosimeter2 = {
  make: { "value": "Marca X", "settings": { "type": "str" } },
  model: { "value": "Modelo Y", "settings": { "type": "str" } },
  sn: { "value": "999999999999", "settings": { "type": "str" } },
  calibration: { "value": "12/01/2019", "settings": { "type": "str" } },
};

let dosimeter3 = {
  make: { "value": "Marca X", "settings": { "type": "str" } },
  model: { "value": "Modelo Y", "settings": { "type": "str" } },
  sn: { "value": "999999999999", "settings": { "type": "str" } },
  calibration: { "value": "12/01/2019", "settings": { "type": "str" } },
};



const dosimeters_data =
{
  headers: [{ title: "Brand" }, { title: "Model" }, { title: "SN" }, { title: "Calibration date" }],
  rows: [dosimeter1, dosimeter2, dosimeter3],
};


export default dosimeters_data