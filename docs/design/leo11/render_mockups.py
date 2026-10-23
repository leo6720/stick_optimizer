from pathlib import Path
from playwright.sync_api import sync_playwright

root=Path(__file__).parent
with sync_playwright() as p:
    browser=p.chromium.launch(executable_path='/usr/bin/chromium',headless=True,args=['--no-sandbox'])
    page=browser.new_page(viewport={'width':1530,'height':1250},device_scale_factor=1)
    errors=[]
    page.on('pageerror',lambda e:errors.append(str(e)))
    page.set_content((root/'confronto-ui.html').read_text(),wait_until='load')
    page.wait_for_function('typeof window.selectMock === "function"')
    page.evaluate('document.fonts.ready')
    for variant in ['A','B','C']:
        for dark in [False,True]:
            page.evaluate('(x)=>window.selectMock(x[0],x[1])',[variant,dark])
            page.locator('#viewport>.mock').screenshot(path=str(root/(variant+'-'+('scuro' if dark else 'chiaro')+'.png')))
    page.evaluate('window.selectMock("A",false)')
    page.locator('#viewport .edit-menu').click()
    assert page.locator('#viewport .theme-menu').is_visible()
    page.locator('#viewport .toggle-theme').click()
    assert 'dark' in page.locator('#viewport>.mock').get_attribute('class')
    assert not page.locator('#viewport .theme-menu').is_visible()
    page.locator('#kit').screenshot(path=str(root/'componenti-A-scuro.png'))
    assert len(page.locator('#viewport .datatable').all())==2
    for theme in [False,True]:
        positions=[]
        for variant in ['A','B','C']:
            page.evaluate('(x)=>window.selectMock(x[0],x[1])',[variant,theme])
            positions.append([page.locator('#viewport '+s).bounding_box() for s in ['.sidebar','.calcbar','.filters','.results .datatable','.tabs']])
        for j in range(len(positions[0])):
            assert max(x[j]['x'] for x in positions)-min(x[j]['x'] for x in positions)<1
            assert max(x[j]['y'] for x in positions)-min(x[j]['y'] for x in positions)<3
    page.evaluate('document.body.classList.add("only-board")')
    page.set_viewport_size({'width':2271,'height':1040})
    page.locator('#boards').screenshot(path=str(root/'confronto-6-varianti.png'))
    assert not errors,errors
    browser.close()
print('6 screenshot + board + componenti; menu/tema e assi di layout tra varianti verificati; zero errori JS.')
