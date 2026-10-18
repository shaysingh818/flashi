import { createApp } from "vue";
import App from "./App.vue";
import { library } from "@fortawesome/fontawesome-svg-core";
import {
  faArrowDown19,
  faDownload,
  faFileExport,
  faHouse,
  faListCheck,
  faListUl,
  faLock,
  faPen,
  faSquareRootVariable,
} from "@fortawesome/free-solid-svg-icons";
import { faCalendar, faChartBar, faClone, faStar } from "@fortawesome/free-regular-svg-icons";
import { faApple, faGithub, faLinux, faWindows } from "@fortawesome/free-brands-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import "./index.css";

import "@graphyne/web-components/style.css";

// Register only the icons the site uses to keep the bundle small.
library.add(
  faArrowDown19, faDownload, faFileExport, faHouse, faListCheck, faListUl, faLock, faPen, faSquareRootVariable,
  faCalendar, faChartBar, faClone, faStar,
  faApple, faGithub, faLinux, faWindows,
);

createApp(App)
  .component("font-awesome-icon", FontAwesomeIcon)
  .mount("#app");
