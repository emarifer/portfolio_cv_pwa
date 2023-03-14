use yew::{function_component, html, Html, Properties, UseStateHandle};

#[derive(Properties, PartialEq)]
pub struct CVDataProps {
    pub activetab: UseStateHandle<i32>,
}

#[function_component(CVData)]
pub fn cv_data(props: &CVDataProps) -> Html {
    html! {
        <>
           /* Datos Personales */
           <article class={format!("{}", if *props.activetab == 1 {""} else {"hidden"})}>
             <h3 class="text-sky-500 font-bold text-lg">{"Mis Datos"}</h3>
             <ul>
               <li class="flex items-start py-2">
                 <span class="font-bold pr-2 text-sky-500">{"▷"}</span>
                 <p class="text-cyan-50 flex flex-wrap text-sm md:px-5 md:text-lg">
                   <span class="font-bold pr-2 text-sky-500">{"Nombre:"}</span>
                   {"Enrique Antonio Marín Fernández"}
                 </p>
               </li>
               <li class="flex items-start py-2">
                 <span class="font-bold pr-2 text-sky-500">{"▷"}</span>
                 <p class="text-cyan-50 flex flex-wrap text-sm md:px-5 md:text-lg">
                   <span class="font-bold pr-2 text-sky-500">{"Fecha de Nacimiento:"}</span>
                   {"04/02/1964"}
                 </p>
               </li>
               <li class="flex items-start py-2">
                 <span class="font-bold pr-2 text-sky-500">{"▷"}</span>
                 <p class="text-cyan-50 flex flex-wrap text-sm md:px-5 md:text-lg">
                   <span class="font-bold pr-2 text-sky-500">{"Nacido en:"}</span>
                   {"Sevilla"}
                 </p>
               </li>
               <li class="flex items-start py-2">
                 <span class="font-bold pr-2 text-sky-500">{"▷"}</span>
                 <p class="text-cyan-50 flex flex-wrap text-sm md:px-5 md:text-lg">
                   <span class="font-bold pr-2 text-sky-500">{"Vivienda Habitual:"}</span>
                   {"C/. Cruz, 1 - 2ºH, 41013 Sevilla"}
                 </p>
               </li>
               <li class="flex items-start py-2">
                 <span class="font-bold pr-2 text-sky-500">{"▷"}</span>
                 <p class="text-cyan-50 flex flex-wrap text-sm md:px-5 md:text-lg">
                   <span class="font-bold pr-2 text-sky-500">{"Estado Civil:"}</span>
                   {"soltero"}
                 </p>
               </li>
               <li class="flex items-start py-2">
                 <span class="font-bold pr-2 text-sky-500">{"▷"}</span>
                 <p class="text-cyan-50 flex flex-wrap text-sm md:px-5 md:text-lg">
                   <span class="font-bold pr-2 text-sky-500">{"DNI:"}</span>
                   {"28699159-N"}
                 </p>
               </li>
               <li class="flex items-start py-2">
                 <span class="font-bold pr-2 text-sky-500">{"▷"}</span>
                 <p class="text-cyan-50 flex flex-wrap text-sm md:px-5 md:text-lg">
                   <span class="font-bold pr-2 text-sky-500">{"Aficiones:"}</span>
                   {"Senderismo, Informática, Leer"}
                 </p>
               </li>
               <li class="flex items-start py-2">
                 <span class="font-bold pr-2 text-sky-500">{"▷"}</span>
                 <p class="text-cyan-50 flex flex-wrap text-sm md:px-5 md:text-lg">
                   <span class="font-bold pr-2 text-sky-500">{"Email:"}</span>
                   {"enriquemarin_sierra@hotmail.com"}
                 </p>
               </li>
               <li class="flex items-start py-2">
                 <span class="font-bold pr-2 text-sky-500">{"▷"}</span>
                 <p class="text-cyan-50 flex flex-wrap text-sm md:px-5 md:text-lg">
                   <span class="font-bold pr-2 text-sky-500">{"Tfno:"}</span>
                   {"615 26 93 82"}
                 </p>
               </li>
             </ul>
           </article>

           /* Datos Académicos */
           <article class={format!("{}", if *props.activetab == 2 {""} else {"hidden"})}>
             <h3 class="text-sky-500 font-bold text-lg">{"Formación"}</h3>
             <ul>
               <li class="flex items-start py-2">
                 <span class="font-bold pr-2 text-sky-500">{"▷"}</span>
                 <p class="text-cyan-50 flex flex-wrap text-sm md:px-5 md:text-lg">
                   <span class="font-bold pr-2 text-sky-500">{"Titulación:"}</span>
                   {"Ingeniería Técnica Agrícola (1988, Universidad de Sevilla)"}
                 </p>
               </li>
               <li class="flex items-start py-2">
                 <span class="font-bold pr-2 text-sky-500">{"▷"}</span>
                 <p class="text-cyan-50 flex flex-wrap text-sm md:px-5 md:text-lg">
                   <span class="font-bold pr-2 text-sky-500">{"Lenguajes de Programación:"}</span>
                   {"HTML, CSS, JavaScript, TypeScript, Rust, WebAssembly, Yew, Tauri, React.js, Vue.js, Node.js, Python, Java, Kotlin, \
                   Desarrollo Frontend & Backend, Curso «React Pro: Lleva tus bases al siguiente nivel» (Udemy), \
                   Curso Deusto de Desarrollo de Aplicaciones Móviles en Android"}
                 </p>
               </li>
               <li class="flex items-start py-2">
                 <span class="font-bold pr-2 text-sky-500">{"▷"}</span>
                 <p class="text-cyan-50 flex flex-wrap text-sm md:px-5 md:text-lg">
                   <span class="font-bold pr-2 text-sky-500">{"Tratamiento de Imágenes y Dibujo:"}</span>
                   {"Adobe Illustrator, Adobe PhotoShop, Gimp, Inkscape"}
                 </p>
               </li>
               <li class="flex items-start py-2">
                 <span class="font-bold pr-2 text-sky-500">{"▷"}</span>
                 <p class="text-cyan-50 flex flex-wrap text-sm md:px-5 md:text-lg">
                   <span class="font-bold pr-2 text-sky-500">{"Cartografía Digital:"}</span>
                   {"ArcGIS, gvSIG"}
                 </p>
               </li>
               <li class="flex items-start py-2">
                 <span class="font-bold pr-2 text-sky-500">{"▷"}</span>
                 <p class="text-cyan-50 flex flex-wrap text-sm md:px-5 md:text-lg">
                   <span class="font-bold pr-2 text-sky-500">{"Maquetación y Edición Digital:"}</span>
                   {"Adobe InDesign, QuarkXPress"}
                 </p>
               </li>
               <li class="flex items-start py-2">
                 <span class="font-bold pr-2 text-sky-500">{"▷"}</span>
                 <p class="text-cyan-50 flex flex-wrap text-sm md:px-5 md:text-lg">
                   <span class="font-bold pr-2 text-sky-500">{"Medio Ambiente:"}</span>
                   {"Monitor de Granja Escuela"}
                 </p>
               </li>
               <li class="flex items-start py-2">
                 <span class="font-bold pr-2 text-sky-500">{"▷"}</span>
                 <p class="text-cyan-50 flex flex-wrap text-sm md:px-5 md:text-lg">
                   <span class="font-bold pr-2 text-sky-500">{"Cerficado de Carretillero:"}</span>
                   {"Carretillas Frontales y Retráctiles"}
                 </p>
               </li>
             </ul>
           </article>

           /* Experiencia Laboral */
           <article class={format!("{}", if *props.activetab == 3 {""} else {"hidden"})}>
             <h3 class="text-sky-500 font-bold text-lg">{"Trabajos Desarrollados"}</h3>
             <ul>
               <li class="flex items-start py-2">
                 <span class="font-bold pr-2 text-sky-500">{"▷"}</span>
                 <p class="text-cyan-50 flex flex-wrap text-sm md:px-5 md:text-lg">
                   <span class="font-bold pr-2 text-sky-500">{"Vida Laboral: 35 años."}</span>
                   {"Desde 1985 a 2008 trabajando en mi propio negocio, LA CATALANA, establecimiento \
                   dedicado a la venta de sombreros, efectos militares y uniformes. Desde 2008 a 2011 \
                   trabajando en la Editorial LA SERRANÍA, editorial líder en Andalucía en la publicación de libros \
                   y guías de Naturaleza, excursionismo, otnitología, micología y otras actividades \
                   ligadas al medio ambiente desarrolladas en los espacios naturales andaluces. \
                   Desde el año 2019 al 2022, en el Real Club Labradores de Sevilla \
                   al cuidado del servicio de piscina y vestuario."}
                 </p>
               </li>
               <li class="flex items-start py-2">
                 <span class="font-bold pr-2 text-sky-500">{"▷"}</span>
                 <p class="text-cyan-50 flex flex-wrap text-sm md:px-5 md:text-lg">
                   <span class="font-bold pr-2 text-sky-500">{"Detalle del Trabajo:"}</span>
                   {"Maquetación y diseño interior de publicaciones. Diseño y maquetación de publicaciones electrónicas (ebooks). \
                   Diseño, Maquetación y Desarrollo Web. Tratamiento de imágenes y dibujo. \
                   Elaboración de cartografías (topográfica, temática, 3D, etc.) y \
                   manejo de Sistemas de Información Geográfica. Corrección de textos (ortografía y estilo). \
                   Corrección y verificación de los contenidos de los textos en materia de geografía y medio natural. \
                   En el Club Labradores, cuidado y atencion al público en las instalaciones de la piscina y vestuario."}
                 </p>
               </li>
             </ul>
           </article>

           /* Autor de Libros */
           <article class={format!("{}", if *props.activetab == 4 {""} else {"hidden"})}>
             <h3 class="text-sky-500 font-bold text-lg">{"Mis Libros"}</h3>
             <ul>
               <li class="flex items-start py-2">
                 <span class="font-bold pr-2 text-sky-500">{"▷"}</span>
                 <p class="text-cyan-50 flex flex-wrap text-sm md:px-5 md:text-lg">
                   <a
                     class="font-bold hover:text-teal-600 ease-in duration-300"
                     href="https://www.laserrania.org/autor/enrique-marin-fernandez/"
                     target="_blank"
                   >
                     {"Autor de Libros relacionados con el Excursionismo:"}
                   </a>
                 </p>
               </li>
               <li class="flex items-start py-2">
                 <span class="font-bold pr-2 text-sky-500">{"▷"}</span>
                 <p class="text-cyan-50 flex flex-wrap text-sm md:px-5 md:text-lg">
                   <a
                     class="font-bold hover:text-teal-600 ease-in duration-300"
                     href="https://www.laserrania.org/producto/sierras-de-cazorla-segura-y-las-villas-guia-del-excursionista/"
                     target="_blank"
                   >
                     {"Parque Natural Sierras de Cazorla, Segura y Las Villas. Guía del Excursionista. Ed. La Serranía, Ronda, 2010"}
                   </a>
                 </p>
               </li>
               <li class="flex items-start py-2">
                 <span class="font-bold pr-2 text-sky-500">{"▷"}</span>
                 <p class="text-cyan-50 flex flex-wrap text-sm md:px-5 md:text-lg">
                   <a
                     class="font-bold hover:text-teal-600 ease-in duration-300"
                     href="https://www.laserrania.org/producto/parque-natural-sierra-de-aracena-y-picos-de-aroche-guia-del-excursionista/"
                     target="_blank"
                   >
                     {"Parque Natural Sierra de Aracena y Picos de Aroche. Guía del Excursionista. Ed. La Serranía, Ronda, 2009"}
                   </a>
                 </p>
               </li>
               <li class="flex items-start py-2">
                 <span class="font-bold pr-2 text-sky-500">{"▷"}</span>
                 <p class="text-cyan-50 flex flex-wrap text-sm md:px-5 md:text-lg">
                   <a
                     class="font-bold hover:text-teal-600 ease-in duration-300"
                     href="https://www.laserrania.org/producto/sierras-andaluzas-itinerarios-senderistas-y-ascensiones-2a-ed/"
                     target="_blank"
                   >
                     {"Sierras Andaluzas. Itinerarios Senderistas y Ascensiones. Ed. La Serranía, Ronda, 2006"}
                   </a>
                 </p>
               </li>
             </ul>
           </article>

           /* Aptitudes */
           <article class={format!("{}", if *props.activetab == 5 {""} else {"hidden"})}>
             <h3 class="text-sky-500 font-bold text-lg">{"En Resumen…"}</h3>
             <ul>
               <li class="flex items-start py-2">
                 <span class="font-bold pr-2 text-sky-500">{"▷"}</span>
                 <p class="text-cyan-50 flex flex-wrap text-sm md:px-5 md:text-lg">
                   <span class="font-bold pr-2 text-sky-500">{"Me considero:"}</span>
                   {"Una persona honesta y muy responsable y trabajadora, con dedicación \
                   plena a mi trabajo además de buena capacidad para comunicar y trabajar en equipo."}
                 </p>
               </li>
             </ul>
           </article>
       </>

    }
}
