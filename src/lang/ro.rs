lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Stare"),
        ("Your Desktop", "Desktopul tău"),
        ("desk_tip", "Desktopul tău poate fi accesat folosind ID-ul și parola de mai jos."),
        ("Password", "Parola"),
        ("Ready", "Pregătit"),
        ("Established", "Stabilit"),
        ("connecting_status", "În curs de conectare la rețeaua RustDesk..."),
        ("Enable Service", "Activează serviciu"),
        ("Start Service", "Pornește serviciu"),
        ("Service is running", "Serviciul este în curs de executare..."),
        ("Service is not running", "Serviciul nu funcționează"),
        ("not_ready_status", "Nepregătit. Verifică conexiunea la rețea."),
        ("Control Remote Desktop", "Controlează desktop-ul la distanță"),
        ("Transfer File", "Transferă fișier"),
        ("Connect", "Conectează-te"),
        ("Recent Sessions", "Sesiuni recente"),
        ("Address Book", "Agendă"),
        ("Confirmation", "Confirmare"),
        ("TCP Tunneling", "Tunel TCP"),
        ("Remove", "Elimină"),
        ("Refresh random password", "Actualizează parolă aleatorie"),
        ("Set your own password", "Setează propria parolă"),
        ("Enable Keyboard/Mouse", "Activează control tastatură/mouse"),
        ("Enable Clipboard", "Activează clipboard"),
        ("Enable File Transfer", "Activează transfer fișiere"),
        ("Enable TCP Tunneling", "Activează tunel TCP"),
        ("IP Whitelisting", "Listă de IP-uri autorizate"),
        ("ID/Relay Server", "Server de ID/retransmisie"),
        ("Import Server Config", "Importă configurație server"),
        ("Export Server Config", "Exportă configurație server"),
        ("Import server configuration successfully", "Configurație server importată cu succes"),
        ("Export server configuration successfully", "Configurație server exportată cu succes"),
        ("Invalid server configuration", "Configurație server nevalidă"),
        ("Clipboard is empty", "Clipboard gol"),
        ("Stop service", "Oprește serviciu"),
        ("Change ID", "Schimbă ID"),
        ("Your new ID", ""),
        ("length %min% to %max%", ""),
        ("starts with a letter", ""),
        ("allowed characters", ""),
        ("id_change_tip", "Pot fi utilizate doar caractere a-z, A-Z, 0-9, _ (bară jos). Primul caracter trebuie să fie a-z, A-Z. Lungimea trebuie să fie între 6 și 16 caractere."),
        ("Website", "Site web"),
        ("About", "Despre"),
        ("Slogan_tip", ""),
        ("Privacy Statement", ""),
        ("Mute", "Fără sunet"),
        ("Build Date", ""),
        ("Version", ""),
        ("Home", ""),
        ("Audio Input", "Intrare audio"),
        ("Enhancements", "Îmbunătățiri"),
        ("Hardware Codec", "Codec hardware"),
        ("Adaptive Bitrate", "Rată de biți adaptabilă"),
        ("ID Server", "Server de ID"),
        ("Relay Server", "Server de retransmisie"),
        ("API Server", "Server API"),
        ("invalid_http", "Trebuie să înceapă cu http:// sau https://"),
        ("Invalid IP", "IP nevalid"),
        ("Invalid format", "Format nevalid"),
        ("server_not_support", "Încă nu este compatibil cu serverul"),
        ("Not available", "Indisponibil"),
        ("Too frequent", "Modificat prea frecvent"),
        ("Cancel", "Anulează"),
        ("Skip", "Omite"),
        ("Close", "Închide"),
        ("Retry", "Reîncearcă"),
        ("OK", "OK"),
        ("Password Required", "Parolă necesară"),
        ("Please enter your password", "Introdu parola"),
        ("Remember password", "Memorează parola"),
        ("Wrong Password", "Parolă incorectă"),
        ("Do you want to enter again?", "Vrei să intri din nou?"),
        ("Connection Error", "Eroare de conexiune"),
        ("Error", "Eroare"),
        ("Reset by the peer", "Conexiunea a fost închisă de dispozitivul pereche"),
        ("Connecting...", "Conectare..."),
        ("Connection in progress. Please wait.", "Conectare în curs. Te rugăm așteaptă."),
        ("Please try 1 minute later", "Reîncearcă într-un minut"),
        ("Login Error", "Eroare de autentificare"),
        ("Successful", "Succes"),
        ("Connected, waiting for image...", "Conectat, se așteaptă transmiterea imaginii..."),
        ("Name", "Denumire"),
        ("Type", "Tip"),
        ("Modified", "Modificat"),
        ("Size", "Dimensiune"),
        ("Show Hidden Files", "Afișează fișiere ascunse"),
        ("Receive", "Acceptă"),
        ("Send", "Trimite"),
        ("Refresh File", "Actualizează fișier"),
        ("Local", "Local"),
        ("Remote", "La distanță"),
        ("Remote Computer", "Computer la distanță"),
        ("Local Computer", "Computer local"),
        ("Confirm Delete", "Confirmă ștergerea"),
        ("Delete", "Șterge"),
        ("Properties", "Caracteristici"),
        ("Multi Select", "Alegere multiplă"),
        ("Select All", "Selectează tot"),
        ("Unselect All", "Deselectează tot"),
        ("Empty Directory", "Director gol"),
        ("Not an empty directory", "Directorul nu este gol"),
        ("Are you sure you want to delete this file?", "Sigur vrei să ștergi acest fișier?"),
        ("Are you sure you want to delete this empty directory?", "Sigur vrei să ștergi acest director gol?"),
        ("Are you sure you want to delete the file of this directory?", "Sigur vrei să ștergi fișierul din acest director?"),
        ("Do this for all conflicts", "Aplică pentru toate conflictele"),
        ("This is irreversible!", "Această acțiune este ireversibilă!"),
        ("Deleting", "În curs de ștergere..."),
        ("files", "fișier"),
        ("Waiting", "În așteptare..."),
        ("Finished", "Finalizat"),
        ("Speed", "Viteză"),
        ("Custom Image Quality", "Setează calitatea imaginii"),
        ("Privacy mode", "Mod privat"),
        ("Block user input", "Blochează utilizator"),
        ("Unblock user input", "Deblochează utilizator"),
        ("Adjust Window", "Ajustează fereastra"),
        ("Original", "Dimensiune originală"),
        ("Shrink", "Micșorează"),
        ("Stretch", "Extinde"),
        ("Scrollbar", "Bară de derulare"),
        ("ScrollAuto", "Derulare automată"),
        ("Good image quality", "Calitate bună a imaginii"),
        ("Balanced", "Calitate normală a imaginii"),
        ("Optimize reaction time", "Optimizează timpul de reacție"),
        ("Custom", "Personalizat"),
        ("Show remote cursor", "Afișează cursor la distanță"),
        ("Show quality monitor", "Afișează indicator de calitate"),
        ("Disable clipboard", "Dezactivează clipboard"),
        ("Lock after session end", "Blochează după deconectare"),
        ("Insert", "Introdu"),
        ("Insert Lock", "Blochează computer"),
        ("Refresh", "Reîmprospătează"),
        ("ID does not exist", "ID neexistent"),
        ("Failed to connect to rendezvous server", "Conectare la server rendezvous eșuată"),
        ("Please try later", "Încearcă mai târziu"),
        ("Remote desktop is offline", "Desktopul la distanță este offline"),
        ("Key mismatch", "Nepotrivire chei"),
        ("Timeout", "Conexiune expirată"),
        ("Failed to connect to relay server", "Conectare la server de retransmisie eșuată"),
        ("Failed to connect via rendezvous server", "Conectare prin intermediul serverului rendezvous eșuată"),
        ("Failed to connect via relay server", "Conectare prin intermediul serverului de retransmisie eșuată"),
        ("Failed to make direct connection to remote desktop", "Imposibil de stabilit o conexiune directă cu desktopul la distanță"),
        ("Set Password", "Setează parola"),
        ("OS Password", "Parolă OS"),
        ("install_tip", "Din cauza restricțiilor UAC, e posibil ca RustDesk să nu funcționeze corespunzător. Pentru a evita acest lucru, fă clic pe butonul de mai jos pentru a instala RustDesk."),
        ("Click to upgrade", "Fă clic pentru a face upgrade"),
        ("Click to download", "Fă clic pentru a descărca"),
        ("Click to update", "Fă clic pentru a actualiza"),
        ("Configure", "Configurează"),
        ("config_acc", "Pentru a controla desktopul la distanță, trebuie să permiți RustDesk acces la setările de Accesibilitate."),
        ("config_screen", "Pentru a controla desktopul la distanță, trebuie să permiți RustDesk acces la setările de Înregistrare ecran."),
        ("Installing ...", "Instalare în curs..."),
        ("Install", "Instalează"),
        ("Installation", "Instalare"),
        ("Installation Path", "Cale de instalare"),
        ("Create start menu shortcuts", "Creează comenzi rapide în meniul Start"),
        ("Create desktop icon", "Creează pictogramă pe desktop"),
        ("agreement_tip", "Începerea procesului de instalare înseamnă acceptarea acordului de licență."),
        ("Accept and Install", "Acceptă și instalează"),
        ("End-user license agreement", "Acord de licență pentru utilizatorul final"),
        ("Generating ...", "Se generează..."),
        ("Your installation is lower version.", "Versiunea instalată este una inferioară."),
        ("not_close_tcp_tip", "Nu închide această fereastră în timp ce folosești tunelul"),
        ("Listening ...", "În așteptarea conexiunii tunel..."),
        ("Remote Host", "Gazdă la distanță"),
        ("Remote Port", "Port la distanță"),
        ("Action", "Acțiune"),
        ("Add", "Adaugă"),
        ("Local Port", "Port local"),
        ("Local Address", "Adresă locală"),
        ("Change Local Port", "Schimbă port local"),
        ("setup_server_tip", "Pentru o conexiune mai rapidă, îți poți configura propriul server."),
        ("Too short, at least 6 characters.", "Prea scurt; trebuie cel puțin 6 caractere."),
        ("The confirmation is not identical.", "Cele două intrări nu corespund."),
        ("Permissions", "Permisiuni"),
        ("Accept", "Acceptă"),
        ("Dismiss", "Respinge"),
        ("Disconnect", "Deconectează-te"),
        ("Allow using keyboard and mouse", "Permite utilizarea tastaturii și mouse-ului"),
        ("Allow using clipboard", "Permite utilizarea clipboardului"),
        ("Allow hearing sound", "Permite auzirea sunetului"),
        ("Allow file copy and paste", "Permite copierea/lipirea fișierelor"),
        ("Connected", "Conectat"),
        ("Direct and encrypted connection", "Conexiune directă criptată"),
        ("Relayed and encrypted connection", "Conexiune retransmisă criptată"),
        ("Direct and unencrypted connection", "Conexiune directă necriptată"),
        ("Relayed and unencrypted connection", "Conexiune retransmisă necriptată"),
        ("Enter Remote ID", "Introdu ID-ul dispozitivului la distanță"),
        ("Enter your password", "Introdu parola"),
        ("Logging in...", "Se conectează..."),
        ("Enable RDP session sharing", "Activează partajarea sesiunii RDP"),
        ("Auto Login", "Conectare automată (valid doar dacă funcția de Blocare după deconectare este activă)"),
        ("Enable Direct IP Access", "Activează accesul direct cu IP"),
        ("Rename", "Redenumește"),
        ("Space", "Spațiu"),
        ("Create Desktop Shortcut", "Creează comandă rapidă de desktop"),
        ("Change Path", "Schimbă calea"),
        ("Create Folder", "Creează folder"),
        ("Please enter the folder name", "Introdu numele folderului"),
        ("Fix it", "Repară"),
        ("Warning", "Avertisment"),
        ("Login screen using Wayland is not supported", "Ecranele de conectare care folosesc Wayland nu sunt acceptate"),
        ("Reboot required", "Repornire necesară"),
        ("Unsupported display server", "Tipul de server de afișaj nu este acceptat"),
        ("x11 expected", "E necesar X11"),
        ("Port", "Port"),
        ("Settings", "Setări"),
        ("Username", " Nume de utilizator"),
        ("Invalid port", "Port nevalid"),
        ("Closed manually by the peer", "Închis manual de dispozitivul pereche"),
        ("Enable remote configuration modification", "Activează modificarea configurației de la distanță"),
        ("Run without install", "Rulează fără instalare"),
        ("Connect via relay", ""),
        ("Always connect via relay", "Se conectează mereu prin retransmisie"),
        ("whitelist_tip", "Doar adresele IP autorizate pot accesa acest dispozitiv"),
        ("Login", "Conectare"),
        ("Verify", ""),
        ("Remember me", ""),
        ("Trust this device", ""),
        ("Verification code", ""),
        ("verification_tip", ""),
        ("Logout", "Deconectare"),
        ("Tags", "Etichetare"),
        ("Search ID", "Caută după ID"),
        ("whitelist_sep", "Poți folosi ca separator virgula, punctul și virgula, spațiul sau linia nouă"),
        ("Add ID", "Adaugă ID"),
        ("Add Tag", "Adaugă etichetă"),
        ("Unselect all tags", "Deselectează toate etichetele"),
        ("Network error", "Eroare de rețea"),
        ("Username missed", "Lipsește numele de utilizator"),
        ("Password missed", "Lipsește parola"),
        ("Wrong credentials", "Nume sau parolă greșită"),
        ("Edit Tag", "Modifică etichetă"),
        ("Unremember Password", "Uită parola"),
        ("Favorites", "Favorite"),
        ("Add to Favorites", "Adaugă la Favorite"),
        ("Remove from Favorites", "Șterge din Favorite"),
        ("Empty", "Gol"),
        ("Invalid folder name", "Denumire folder nevalidă"),
        ("Socks5 Proxy", "Proxy Socks5"),
        ("Hostname", "Nume gazdă"),
        ("Discovered", "Descoperite"),
        ("install_daemon_tip", "Pentru executare la pornirea sistemului, instalează serviciul de sistem."),
        ("Remote ID", "ID dispozitiv la distanță"),
        ("Paste", "Lipește"),
        ("Paste here?", "Lipește aici?"),
        ("Are you sure to close the connection?", "Sigur vrei să închizi conexiunea?"),
        ("Download new version", "Descarcă noua versiune"),
        ("Touch mode", "Mod tactil"),
        ("Mouse mode", "Mod mouse"),
        ("One-Finger Tap", "Apasă cu un deget"),
        ("Left Mouse", "Clic stânga"),
        ("One-Long Tap", "Apasă lung"),
        ("Two-Finger Tap", "Apasă cu două degete"),
        ("Right Mouse", "Clic dreapta"),
        ("One-Finger Move", "Mișcă cu un deget"),
        ("Double Tap & Move", "Apasă dublu și mișcă"),
        ("Mouse Drag", "Tragere mouse"),
        ("Three-Finger vertically", "Trei degete vertical"),
        ("Mouse Wheel", "Rotiță mouse"),
        ("Two-Finger Move", "Mișcă cu două degete"),
        ("Canvas Move", "Mută ecran"),
        ("Pinch to Zoom", "Apropie degetele pentru zoom"),
        ("Canvas Zoom", "Zoom ecran"),
        ("Reset canvas", "Reinițializează ecranul"),
        ("No permission of file transfer", "Nicio permisiune pentru transferul de fișiere"),
        ("Note", "Reține"),
        ("Connection", "Conexiune"),
        ("Share Screen", "Partajează ecran"),
        ("CLOSE", "ÎNCHIDE"),
        ("OPEN", "DESCHIDE"),
        ("Chat", "Discută"),
        ("Total", "Total"),
        ("items", "elemente"),
        ("Selected", "Selectat"),
        ("Screen Capture", "Captură ecran"),
        ("Input Control", "Control intrări"),
        ("Audio Capture", "Captură audio"),
        ("File Connection", "Conexiune fișier"),
        ("Screen Connection", "Conexiune ecran"),
        ("Do you accept?", "Accepți?"),
        ("Open System Setting", "Deschide setări sistem"),
        ("How to get Android input permission?", "Cum autorizez dispozitive de intrare pe Android?"),
        ("android_input_permission_tip1", "Pentru ca un dispozitiv la distanță să poată controla un dispozitiv Android folosind mouse-ul sau suportul tactil, trebuie să permiți RustDesk să utilize serviciul Accesibilitate."),
        ("android_input_permission_tip2", "Accesează următoarea pagină din Setări, caută și deschide [Aplicații instalate] și activează serviciul [RustDesk Input]."),
        ("android_new_connection_tip", "Ai primit o nouă solicitare de control pentru dispozitivul actual."),
        ("android_service_will_start_tip", "Activarea setării Captură ecran va porni automat serviciul, permițând altor dispozitive să solicite conectarea la dispozitivul tău."),
        ("android_stop_service_tip", "Închiderea serviciului va închide automat toate conexiunile stabilite."),
        ("android_version_audio_tip", "Versiunea actuală de Android nu suportă captura audio. Fă upgrade la Android 10 sau la o versiune superioară."),
        ("android_start_service_tip", "Apasă [Pornește serviciu] sau DESCHIDE [Captură ecran] pentru a porni serviciul de partajare a ecranului."),
        ("Account", "Cont"),
        ("Overwrite", "Suprascrie"),
        ("This file exists, skip or overwrite this file?", "Fișier deja existent. Omite sau suprascrie?"),
        ("Quit", "Ieși"),
        ("doc_mac_permission", "https://rustdesk.com/docs/en/manual/mac/#enable-permissions"),
        ("Help", "Ajutor"),
        ("Failed", "Nereușit"),
        ("Succeeded", "Reușit"),
        ("Someone turns on privacy mode, exit", "Cineva activează modul privat, ieși din"),
        ("Unsupported", "Neacceptat"),
        ("Peer denied", "Dispozitiv pereche refuzat"),
        ("Please install plugins", "Instalează pluginuri"),
        ("Peer exit", "Ieșire dispozitiv pereche"),
        ("Failed to turn off", "Dezactivare nereușită"),
        ("Turned off", "Închis"),
        ("In privacy mode", "În modul privat"),
        ("Out privacy mode", "Ieșit din modul privat"),
        ("Language", "Limbă"),
        ("Keep RustDesk background service", "Rulează serviciul RustDesk în fundal"),
        ("Ignore Battery Optimizations", "Ignoră optimizările de baterie"),
        ("android_open_battery_optimizations_tip", "Pentru dezactivarea acestei funcții, accesează setările aplicației RustDesk, deschide secțiunea [Baterie] și deselectează [Fără restricții]."),
        ("Start on Boot", ""),
        ("Start the screen sharing service on boot, requires special permissions", ""),
        ("Connection not allowed", "Conexiune neautoriztă"),
        ("Legacy mode", "Mod legacy"),
        ("Map mode", "Mod hartă"),
        ("Translate mode", "Mod traducere"),
        ("Use permanent password", "Folosește parola permanentă"),
        ("Use both passwords", "Folosește parola unică și cea permanentă"),
        ("Set permanent password", "Setează parola permanentă"),
        ("Enable Remote Restart", "Activează repornirea la distanță"),
        ("Allow remote restart", "Permite repornirea la distanță"),
        ("Restart Remote Device", "Repornește dispozivul la distanță"),
        ("Are you sure you want to restart", "Sigur vrei să repornești dispozitivul?"),
        ("Restarting Remote Device", "Se repornește dispozitivul la distanță"),
        ("remote_restarting_tip", "Dispozitivul este în curs de repornire. Închide acest mesaj și reconectează-te cu parola permanentă după un timp."),
        ("Copied", "Copiat"),
        ("Exit Fullscreen", "Ieși din modul ecran complet"),
        ("Fullscreen", "Ecran complet"),
        ("Mobile Actions", "Acțiuni mobile"),
        ("Select Monitor", "Selectează monitor"),
        ("Control Actions", "Acțiuni de control"),
        ("Display Settings", "Setări afișaj"),
        ("Ratio", "Raport"),
        ("Image Quality", "Calitate imagine"),
        ("Scroll Style", "Stil de derulare"),
        ("Show Menubar", "Arată bara de meniu"),
        ("Hide Menubar", "Ascunde bara de meniu"),
        ("Direct Connection", "Conexiune directă"),
        ("Relay Connection", "Conexiune prin retransmisie"),
        ("Secure Connection", "Conexiune securizată"),
        ("Insecure Connection", "Conexiune nesecurizată"),
        ("Scale original", "Scală originală"),
        ("Scale adaptive", "Scală adaptivă"),
        ("General", "General"),
        ("Security", "Securitate"),
        ("Theme", "Temă"),
        ("Dark Theme", "Temă întunecată"),
        ("Light Theme", ""),
        ("Dark", "Întunecat"),
        ("Light", "Luminos"),
        ("Follow System", "Urmărește sistem"),
        ("Enable hardware codec", "Activează codec hardware"),
        ("Unlock Security Settings", "Deblochează setări de securitate"),
        ("Enable Audio", "Activează audio"),
        ("Unlock Network Settings", "Deblochează setări de rețea"),
        ("Server", "Server"),
        ("Direct IP Access", "Acces direct IP"),
        ("Proxy", "Proxy"),
        ("Apply", "Aplică"),
        ("Disconnect all devices?", "Vrei să deconectezi toate dispozitivele?"),
        ("Clear", "Golește"),
        ("Audio Input Device", "Dispozitiv de intrare audio"),
        ("Deny remote access", "Interzice acces la distanță"),
        ("Use IP Whitelisting", "Folosește lista de IP-uri autorizate"),
        ("Network", "Rețea"),
        ("Enable RDP", "Activează RDP"),
        ("Pin menubar", "Fixează bara de meniu"),
        ("Unpin menubar", "Detașează bara de meniu"),
        ("Recording", "Înregistrare"),
        ("Directory", "Director"),
        ("Automatically record incoming sessions", "Înregistrează automat sesiunile viitoare"),
        ("Change", "Modifică"),
        ("Start session recording", "Începe înregistrare"),
        ("Stop session recording", "Oprește înregistrare"),
        ("Enable Recording Session", "Activează înregistrarea sesiunii"),
        ("Allow recording session", "Permite înregistrarea sesiunii"),
        ("Enable LAN Discovery", "Activează descoperire LAN"),
        ("Deny LAN Discovery", "Interzice descoperire LAN"),
        ("Write a message", "Scrie un mesaj"),
        ("Prompt", "Solicită"),
        ("Please wait for confirmation of UAC...", "Așteaptă confirmarea UAC..."),
        ("elevated_foreground_window_tip", "Fereastra actuală a dispozitivului la distanță necesită privilegii sporite pentru a funcționa, astfel că mouse-ul și tastatura nu pot fi folosite. Poți cere utilizatorului la distanță să minimizeze fereastra actuală sau să facă clic pe butonul de sporire a privilegiilor din fereastra de gestionare a conexiunilor. Pentru a evita această problemă, recomandăm instalarea software-ului pe dispozitivul la distanță."),
        ("Disconnected", "Deconectat"),
        ("Other", "Altele"),
        ("Confirm before closing multiple tabs", "Confirmă înainte de a închide mai multe file"),
        ("Keyboard Settings", "Configurare tastatură"),
        ("Full Access", "Acces total"),
        ("Screen Share", "Partajare ecran"),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland necesită Ubuntu 21.04 sau o versiune superioară."),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Wayland necesită o versiune superioară a distribuției Linux. Încearcă desktopul X11 sau schimbă sistemul de operare."),
        ("JumpLink", "Afișează"),
        ("Please Select the screen to be shared(Operate on the peer side).", "Partajează ecranul care urmează să fie partajat (operează din partea dispozitivului pereche)."),
        ("Show RustDesk", "Afișează RustDesk"),
        ("This PC", "Acest PC"),
        ("or", "sau"),
        ("Continue with", "Continuă cu"),
        ("Elevate", "Sporește"),
        ("Zoom cursor", "Cursor lupă"),
        ("Accept sessions via password", "Acceptă sesiunile folosind parola"),
        ("Accept sessions via click", "Acceptă sesiunile cu un clic de confirmare"),
        ("Accept sessions via both", "Acceptă sesiunile folosind ambele moduri"),
        ("Please wait for the remote side to accept your session request...", "Așteaptă ca solicitarea ta de conectare la distanță să fie acceptată..."),
        ("One-time Password", "Parolă unică"),
        ("Use one-time password", "Folosește parola unică"),
        ("One-time password length", "Lungimea parolei unice"),
        ("Request access to your device", "Solicită acces la dispozitivul tău"),
        ("Hide connection management window", "Ascunde fereastra de gestionare a conexiunilor"),
        ("hide_cm_tip", "Permite ascunderea ferestrei de gestionare doar dacă accepți începerea sesiunilor folosind parola permanentă"),
        ("wayland_experiment_tip", ""),
        ("Right click to select tabs", ""),
        ("Skipped", ""),
        ("Add to Address Book", ""),
        ("Group", ""),
        ("Search", ""),
        ("Closed manually by web console", ""),
        ("Local keyboard type", ""),
        ("Select local keyboard type", ""),
        ("software_render_tip", ""),
        ("Always use software rendering", ""),
        ("config_input", ""),
        ("config_microphone", ""),
        ("request_elevation_tip", ""),
        ("Wait", ""),
        ("Elevation Error", ""),
        ("Ask the remote user for authentication", ""),
        ("Choose this if the remote account is administrator", ""),
        ("Transmit the username and password of administrator", ""),
        ("still_click_uac_tip", ""),
        ("Request Elevation", ""),
        ("wait_accept_uac_tip", ""),
        ("Elevate successfully", ""),
        ("uppercase", ""),
        ("lowercase", ""),
        ("digit", ""),
        ("special character", ""),
        ("length>=8", ""),
        ("Weak", ""),
        ("Medium", ""),
        ("Strong", ""),
        ("Switch Sides", ""),
        ("Please confirm if you want to share your desktop?", ""),
        ("Display", ""),
        ("Default View Style", ""),
        ("Default Scroll Style", ""),
        ("Default Image Quality", ""),
        ("Default Codec", ""),
        ("Bitrate", ""),
        ("FPS", ""),
        ("Auto", ""),
        ("Other Default Options", ""),
        ("Voice call", ""),
        ("Text chat", ""),
        ("Stop voice call", ""),
        ("relay_hint_tip", ""),
        ("Reconnect", ""),
        ("Codec", ""),
        ("Resolution", ""),
        ("No transfers in progress", ""),
        ("Set one-time password length", ""),
        ("Install driver cert (test cert)", ""),
        ("Virtual display need", ""),
        ("instsall_cert_tip", "")
    ].iter().cloned().collect();
}
