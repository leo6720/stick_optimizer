from pathlib import Path
from playwright.sync_api import sync_playwright
root=Path(__file__).parent
with sync_playwright() as p:
 browser=p.chromium.launch(executable_path='/usr/bin/chromium',headless=True,args=['--no-sandbox'])
 page=browser.new_page(viewport={'width':1530,'height':1250},device_scale_factor=1)
 errors=[]
 page.on('pageerror',lambda e:errors.append(str(e)))
 page.set_content((root/'proposta-v2.html').read_text(),wait_until='load')
 page.evaluate('document.fonts.ready')
 assert page.evaluate('document.fonts.check("14px Inter")')
 for d in [False,True]:
  page.evaluate('(d)=>window.selectMock("A",d)',d)
  page.locator('#viewport>.mock').screenshot(path=str(root/('v2-'+('scuro' if d else 'chiaro')+'.png')))
  page.locator('#kit').screenshot(path=str(root/('v2-componenti-'+('scuro' if d else 'chiaro')+'.png')))
  assert page.locator('#viewport .main').evaluate('(e)=>e.scrollWidth<=e.clientWidth')
  print('Main viewport, contenuto:',page.locator('#viewport .main').evaluate('(e)=>[e.clientHeight,e.scrollHeight]'))
  print('Viewport risultati scroll:',page.locator('#viewport .result-table-frame').evaluate('(e)=>[e.clientHeight,e.scrollHeight]'))
 page.locator('#viewport .edit-menu').click()
 assert page.locator('#viewport .theme-menu').is_visible()
 page.locator('#viewport .toggle-theme').click()
 assert 'dark' not in page.locator('#viewport>.mock').get_attribute('class')
 assert not errors,errors
 page.evaluate('document.body.classList.add("only-board")')
 page.set_viewport_size({'width':1540,'height':550})
 page.locator('#boards').screenshot(path=str(root/'v2-confronto.png'))
 browser.close()
print('V2: font caricato, temi/menu funzionanti;4 catture+confronto; nessun erroreJS.')
