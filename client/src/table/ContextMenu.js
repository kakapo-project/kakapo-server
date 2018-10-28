
import React, { Component } from 'react'

import { Popup, Portal } from 'semantic-ui-react'

import Ref from 'semantic-ui-react/src/addons/Ref'
import PortalInner from 'semantic-ui-react/src/addons/Portal/PortalInner'

import _ from 'lodash'

class ContextPortal extends Portal {

  handleContextMenu = (e, ...rest) => {
    console.log('handleContextMenu')
  }

  render() {
    const { children, mountNode, trigger } = this.props
    const { open } = this.state

    return (
      <React.Fragment>
        {open && (
          <PortalInner
            mountNode={mountNode}
            onMount={this.handleMount}
            onUnmount={this.handleUnmount}
          >
            {children}
          </PortalInner>
        )}
        {trigger && (
          <Ref innerRef={this.handleTriggerRef}>
            {React.cloneElement(trigger, {
              onBlur: this.handleTriggerBlur,
              onClick: this.handleTriggerClick,
              onContextMenu: this.handleContextMenu,
              onFocus: this.handleTriggerFocus,
              onMouseLeave: this.handleTriggerMouseLeave,
              onMouseEnter: this.handleTriggerMouseEnter,
            })}
          </Ref>
        )}
      </React.Fragment>
    )
  }

}

class ContextMenu extends Popup {

  static defaultProps = {
    ...Popup.defaultProps,
    on: 'click'
  }

  render() {
    let elem = super.render()
    return (
      <ContextPortal {...elem.props}>
        {elem.props.children}
      </ContextPortal>
    )
  }
}

export default ContextMenu