# tp-wik-dps-tp01 - Typescript version
## Build & Run
### Software needed
```
node, npm, git
```
1. Clone the repository
```
git clone https://github.com/arpaflow/tp-wik-dps-tp01.git
```
2. Install dependencies
```
npm install
```
3. Build
```
npx tsc
```
4. Export the PING_LISTEN_PORT environnement variable with the port of your choice
```
export PING_LISTEN_PORT=<port>
```
5. Run
```
node build/index.js
```
6. Test
```
curl http://localhost:<port>/ping
```