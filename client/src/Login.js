import React, { Component } from 'react'
import { Button, Divider, Form, Grid, Header, Image, Message, Segment, Transition } from 'semantic-ui-react'

import logo from './logo.svg'



class LoginForm extends Component {
  render() {
    return (
      <div className='login-form' style={{height: '100vh', background: 'linear-gradient(45deg, #222 0%, #005322 30%, #D8DD87 100%)'}}>
        <style>{`
          body > div,
          body > div > div,
          body > div > div > div.login-form {
            height: 100vh;
          }
        `}</style>
        <Grid textAlign='center' style={{ height: '100%' }} verticalAlign='middle'>
          <Grid.Column style={{ maxWidth: 450 }}>
            <Transition visible transitionOnMount animation='fade' duration={600}>
              <Form size='large'>
                <Segment style={{border: 0, boxShadow: '0px 0px 15px 0px rgba(10, 40, 30, 0.8)'}}>
                  <Header as='h2' color='grey' textAlign='center'>
                    <Image src={logo} /> Log-in to your account
                  </Header>
                  <Form.Input fluid icon='user' iconPosition='left' placeholder='E-mail address' />
                  <Form.Input
                    fluid
                    icon='lock'
                    iconPosition='left'
                    placeholder='Password'
                    type='password'
                  />

                  <Button color='grey' fluid size='large'>
                    Login
                  </Button>
                  <Message>
                    New to us? <a href='#'>Sign Up</a>
                  </Message>
                </Segment>
              </Form>
            </Transition>
          </Grid.Column>
        </Grid>
      </div>
    )
  }
}

export default LoginForm